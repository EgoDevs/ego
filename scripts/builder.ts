import file, { fstat } from 'fs';
import shell from 'shelljs';
import yargs from 'yargs';

import {
  appsConfig,
  infraConfig,
  dfxConfigTemplate,
  ProjectConfig,
} from './config';

interface ThisArgv {
  [x: string]: unknown;
  infra: boolean | undefined;
  apps: boolean | undefined;
  idl: boolean | undefined;
  project: string | undefined;
  _: (string | number)[];
  $0: string;
}

const argv = yargs
  .option('infra', {
    description: 'build infra only',
    type: 'boolean',
  })
  .option('apps', {
    description: 'build apps only',
    type: 'boolean',
  })
  .option('idl', {
    alias: 'i',
    description: 'build idl only',
    type: 'boolean',
  })
  .option('project', {
    alias: 'p',
    description: 'build project only',
    type: 'string',
  })
  .help()
  .alias('help', 'h').argv;

function checkAndArtifacs(ego: ProjectConfig) {
  let folder_exist = true;
  try {
    folder_exist = file.existsSync(`${process.cwd()}/artifacts/${ego.package}`);
  } catch (error) {
    folder_exist = false;
  }

  if (!folder_exist) {
    shell.exec(`mkdir ${process.cwd()}/artifacts/${ego.package}`);
  }
}

function generateDFXJson(ego: ProjectConfig) {
  let shouldSaveName = `${process.cwd()}/artifacts/${ego.package}/dfx.json`;
  shell.exec(`rm -rf ${shouldSaveName}`);
  const packageItem = {};

  packageItem[ego.package] = {
    type: 'custom',
    candid: `${ego.package}.did`,
    wasm: `${ego.package}_opt.wasm`,
    build: [],
  };
  // dfxConfigTemplate.canisters
  dfxConfigTemplate['canisters'] = packageItem;
  // const target = Object.assign(dfxConfigTemplate.canisters, packageItem);
  file.writeFileSync(shouldSaveName, JSON.stringify(dfxConfigTemplate));
}

function buildDID(ego: ProjectConfig) {
  console.log({ ego });
  let originFile = `${process.cwd()}/ego/${ego.category}/${ego.package}/${
    ego.package
  }.did`;

  let shouldSaveAutoName = `${process.cwd()}/artifacts/${ego.package}/${
    ego.package
  }.auto.did`;
  let shouldSaveName = `${process.cwd()}/artifacts/${ego.package}/${
    ego.package
  }.did`;

  if (ego.custom_candid && file.existsSync(originFile)) {
    shell.exec(`cp ${originFile} ${shouldSaveName}`);
  }

  let did_file_exist = true;
  try {
    did_file_exist = file.existsSync(shouldSaveName);
  } catch (error) {
    did_file_exist = false;
  }
  console.log({ did_file_exist });
  if (did_file_exist && ego.custom_candid) {
    shell.exec(`
    EGO_DIR="${process.cwd()}/ego/${ego.category}/${ego.package}"
    cd $EGO_DIR/actor && cargo run ${ego.bin_name} > ${shouldSaveAutoName}
    `);

    const theFile = file.readFileSync(shouldSaveAutoName, { encoding: 'utf8' });
    file.writeFileSync(
      shouldSaveAutoName,
      theFile.replace(
        'service : () -> {\n',
        'type InitArg = record {\n' +
          '  init_caller: opt principal;\n' +
          '};' +
          'service : (InitArg) -> {\n',
      ),
    );
    file.writeFileSync(
      shouldSaveAutoName,
      theFile.replace(
        'service : {\n',
        'type InitArg = record {\n' +
          '  init_caller: opt principal;\n' +
          '};' +
          'service : (InitArg) -> {\n',
      ),
    );
  } else {
    shell.exec(`
    EGO_DIR="${process.cwd()}/ego/${ego.category}/${ego.package}"
    cd $EGO_DIR/actor && cargo run ${
      ego.bin_name
    } > ${shouldSaveAutoName} && cargo run ${ego.bin_name} > ${shouldSaveName}
    `);

    const theFile = file.readFileSync(shouldSaveAutoName, { encoding: 'utf8' });

    const newFile = theFile.replace(
      'service : () -> {\n',
      'type InitArg = record {\n' +
        '  init_caller: opt principal;\n' +
        '};\n' +
        'service : (InitArg) -> {\n',
    );
    const newFile_to = newFile.replace(
      'service : {\n',
      'type InitArg = record {\n' +
        '  init_caller: opt principal;\n' +
        '};\n' +
        'service : (InitArg) -> {\n',
    );

    file.writeFileSync(shouldSaveAutoName, newFile_to);

    const theFile2 = file.readFileSync(shouldSaveName, { encoding: 'utf8' });
    const newFile2 = theFile2.replace(
      'service : () -> {\n',
      'type InitArg = record {\n' +
        '  init_caller: opt principal;\n' +
        '};\n' +
        'service : (InitArg) -> {\n',
    );
    const newFile2_to = newFile2.replace(
      'service : {\n',
      'type InitArg = record {\n' +
        '  init_caller: opt principal;\n' +
        '};\n' +
        'service : (InitArg) -> {\n',
    );

    file.writeFileSync(shouldSaveName, newFile2_to);
  }
}

function buildIDL(ego: ProjectConfig) {
  shell.exec(`
    EGO_DIR="${process.cwd()}/artifacts/${ego.package}"
    didc bind $EGO_DIR/${
      ego.package
    }.did -t ts > ${process.cwd()}/clients/idls/${ego.package}.d.ts
    didc bind $EGO_DIR/${
      ego.package
    }.did -t js > ${process.cwd()}/clients/idls/${ego.package}.idl.ts
    `);
}

function buildExampleIDL(ego: ProjectConfig) {
  shell.exec(`
    EGO_DIR="${process.cwd()}/artifacts/${ego.package}"
    didc bind $EGO_DIR/${
      ego.package
    }.did -t ts > ${process.cwd()}/clients/ego_land/src/canisters/${
    ego.package
  }.d.ts
    didc bind $EGO_DIR/${
      ego.package
    }.did -t js > ${process.cwd()}/clients/ego_land/src/canisters/${
    ego.package
  }.idl.js
    `);
}

function runBuildRust(ego: ProjectConfig) {
  // buildDID();
  if (ego.no_build === false || ego.no_build === undefined) {
    let shouldSaveName = `${process.cwd()}/artifacts/${ego.package}/${
      ego.package
    }_opt.wasm`;

    const constantFile = file
      .readFileSync(process.cwd() + '/configs/constant.json')
      .toString('utf8');

    const staging = JSON.parse(constantFile)['staging'];
    if (staging === 'production') {
      // getSnapshot();
      shell.exec(`
          PARENT_DIR="${process.cwd()}/ego"
          EGO_DIR="${process.cwd()}/ego/${ego.category}/${ego.package}"
          CAT_DIR="${process.cwd()}/ego/${ego.category}"
          TARGET="wasm32-unknown-unknown"
          cargo build --manifest-path "$EGO_DIR/actor/Cargo.toml" --target $TARGET --release -j1
          cargo install ic-cdk-optimizer --version 0.3.2
          STATUS=$?
          echo "$PARENT_DIR/target/$TARGET/release/${ego.package}.wasm"
          if [ "$STATUS" -eq "0" ]; then
                 ic-cdk-optimizer \
                 "$PARENT_DIR/target/$TARGET/release/${ego.package}.wasm" \
                 -o "${shouldSaveName}"
          
             true
           else
             echo Could not install ic-cdk-optimizer.
             false
           fi
          `);
      // buildDID();
    } else {
      // getSnapshot();
      shell.exec(`
          EGO_DIR="${process.cwd()}/ego/${ego.category}/${ego.package}" 
          TARGET="wasm32-unknown-unknown"
          cargo build --manifest-path "$EGO_DIR/actor/Cargo.toml" --target $TARGET --release -j1
          `);
      // buildDID();
    }
  }
}

function runEgoBuilder(): void {
  if ((argv as ThisArgv).infra) {
    infraConfig.forEach(ego => {
      runBuildRust(ego);
      if ((argv as ThisArgv).idl) {
        buildIDL(ego);
      } else {
        buildDID(ego);
        buildIDL(ego);
        // buildExampleIDL(ego);
      }
    });
  }
  if ((argv as ThisArgv).apps) {
    appsConfig.forEach(ego => {
      runBuildRust(ego);
      if ((argv as ThisArgv).idl) {
        buildIDL(ego);
      } else {
        buildDID(ego);
        buildIDL(ego);
        // buildExampleIDL(ego);
      }
    });
  }
  if ((argv as ThisArgv).project) {
    const project = (argv as ThisArgv).project;
    const ego = [...infraConfig, ...appsConfig].find(
      e => e.package === project,
    );
    if (ego) {
      runBuildRust(ego);
      if ((argv as ThisArgv).idl) {
        buildIDL(ego);
      } else {
        buildDID(ego);
        buildIDL(ego);
        // buildExampleIDL(ego);
      }
    }
  }
  if (
    !(argv as ThisArgv).infra &&
    !(argv as ThisArgv).apps &&
    !(argv as ThisArgv).project
  ) {
    [...infraConfig, ...appsConfig].forEach(ego => {
      runBuildRust(ego);
      if ((argv as ThisArgv).idl) {
        buildIDL(ego);
      } else {
        buildDID(ego);
        buildIDL(ego);
        // buildExampleIDL(ego);
      }
    });
  }
}

runEgoBuilder();

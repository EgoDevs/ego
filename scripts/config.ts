export interface ProjectConfig {
  category: string;
  package: string;
  bin_name: string;
  config: string;
  private?: string;
  public?: string;
  url?: string;
  custom_candid?: boolean;
  // if pass true, builder.js won't build, and will ignore all build script
  no_build?: boolean;
  // if pass this script, builder.js will run this shell
  custom_build?: string;
  // if pass true, deployer.js won't deploy, and will ignore all deploy script
  no_deploy?: boolean;
  // if pass this script, deployer.js will run this shell
  custom_deploy?: string | (() => void) | (() => Promise<void>);
  // if pass this script, deployer.js will run this shell after first install/deploy
  post_install_sequence?: number;
}

export type Configs = Array<ProjectConfig>;

export const infraConfig: Configs = [
  {
    category: 'infra',
    package: 'ego_tenant',
    bin_name: 'ego-tenant',
    config: './configs/ego_tenant.json',
    post_install_sequence: 100
  },
  {
    category: 'infra',
    package: 'ego_dev',
    bin_name: 'ego-dev',
    config: './configs/ego_dev.json',
    post_install_sequence: 100
  },
  {
    category: 'infra',
    package: 'ego_file',
    bin_name: 'ego-file',
    config: './configs/ego_file.json',
    post_install_sequence: 100
  },
  {
    category: 'infra',
    package: 'ego_store',
    bin_name: 'ego-store',
    config: './configs/ego_store.json',
    post_install_sequence: 100
  },
  {
    category: 'infra',
    package: 'ego_assets',
    bin_name: 'ego-assets',
    config: './configs/ego_assets.json',
    post_install_sequence: 100,
    no_deploy: true
  },
  {
    category: 'infra',
    package: 'ego_ops',
    bin_name: 'ego-ops',
    config: './configs/ego_ops.json',
    post_install_sequence: 100,
  },
  {
    category: 'infra',
    package: 'ego_ledger',
    bin_name: 'ego-ledger',
    config: './configs/ego_ledger.json',
    post_install_sequence: 100,
  },
  {
    category: 'infra',
    package: 'ego_record',
    bin_name: 'ego-record',
    config: './configs/ego_record.json',
    post_install_sequence: 100,
  },
];

export const appsConfig: Configs = [];

export const dfxConfigTemplate = {
  canisters: {},
  defaults: {
    build: {
      packtool: '',
    },
  },
  networks: {
    local: {
      bind: '127.0.0.1:8080',
      type: 'ephemeral',
    },
    mainnet: {
      providers: ['https://identity.ic0.app'],
      type: 'persistent',
    },
  },
  version: 1,
};

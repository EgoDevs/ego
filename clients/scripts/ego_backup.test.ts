import { idlFactory as Idl } from "../idls/ego_store.idl";
import { _SERVICE as Service } from "../idls/ego_store";

import { getActor, getCanisterId, identity } from "@ego-js/utils";

import fs from "fs";

describe("ego_store_backup", () => {
  it("backup_data", async () => {
    const actor = await getActor<Service>(
      identity(),
      Idl,
      getCanisterId("ego_store")!
    );
    await backup(actor, "/tmp/ego_store");
  });
});

describe("ego_store_restore", () => {
  it("restore_data", async () => {
    const actor = await getActor<Service>(
      identity(),
      Idl,
      getCanisterId("ego_store")!
    );
    await restore(actor, "/tmp/ego_store");
  });
});

describe("ego_tenant_backup", () => {
  it("backup_data", async () => {
    const actor = await getActor<Service>(
      identity(),
      Idl,
      getCanisterId("ego_tenant")!
    );
    await backup(actor, "/tmp/ego_tenant");
  });
});

describe("ego_tenant_restore", () => {
  it("restore_data", async () => {
    const actor = await getActor<Service>(
      identity(),
      Idl,
      getCanisterId("ego_tenant")!
    );
    await restore(actor, "/tmp/ego_tenant");
  });
});

describe("ego_dev_backup", () => {
  it("backup_data", async () => {
    const actor = await getActor<Service>(
      identity(),
      Idl,
      getCanisterId("ego_dev")!
    );
    await backup(actor, "/tmp/ego_dev");
  });
});

describe("ego_dev_restore", () => {
  it("restore_data", async () => {
    const actor = await getActor<Service>(
      identity(),
      Idl,
      getCanisterId("ego_dev")!
    );
    await restore(actor, "/tmp/ego_dev");
  });
});

async function backup(actor, dir_path) {
  const backup_dir_path = `${dir_path}/backup`;
  fs.mkdirSync(backup_dir_path, { recursive: true });

  const restore_dir_path = `${dir_path}/restore`;
  fs.mkdirSync(restore_dir_path, { recursive: true });

  const step = 5000;

  const response = await actor.backup_job_list();
  const job_list = response.Ok;

  for (const job of job_list) {
    const name = job["name"];
    const amount = job["amount"];

    const sub_dir_path = `${backup_dir_path}/${name}`;
    fs.mkdirSync(sub_dir_path, { recursive: true });

    for (let start = 0; start < amount; start += step) {
      let end = start + step;
      if (end > amount) {
        end = amount;
      }

      const file_path = `${sub_dir_path}/${start}.bin`;

      const exists = fs.existsSync(file_path);
      if (!exists) {
        console.log(`backup ${file_path}`);
        const response = await actor.job_data_backup(
          name,
          BigInt(start),
          BigInt(end)
        );
        const data = response.Ok[0].data;
        fs.writeFileSync(file_path, data);
      }
    }
  }
}

async function restore(actor, dir_path) {
  const backup_dir_path = `${dir_path}/backup`;
  fs.mkdirSync(backup_dir_path, { recursive: true });

  const restore_dir_path = `${dir_path}/restore`;
  fs.mkdirSync(restore_dir_path, { recursive: true });

  const response = await actor.backup_job_list();
  const job_list = response.Ok;

  for (const job of job_list) {
    const name = job["name"];

    const sub_backup_dir_path = `${backup_dir_path}/${name}`;
    const sub_restore_dir_path = `${restore_dir_path}/${name}`;
    fs.mkdirSync(sub_restore_dir_path, { recursive: true });

    const files = fs.readdirSync(sub_backup_dir_path);

    for (const file of files) {
      const backup_file_path = `${sub_backup_dir_path}/${file}`;
      const restore_file_path = `${sub_restore_dir_path}/${file}`;
      console.log(`restore ${backup_file_path}`);

      const data = fs.readFileSync(backup_file_path);
      const response = await actor.job_data_restore(name, data);
      console.log(response);
      if (response.hasOwnProperty("Ok")) {
        fs.renameSync(backup_file_path, restore_file_path);
      }
    }
  }
}

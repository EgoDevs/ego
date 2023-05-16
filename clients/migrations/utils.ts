import { ActorSubclass } from '@dfinity/agent';
import fs from 'fs';

export async function data_import<T>(wallet: ActorSubclass<T>, file_path: string) {
  const data = fs.readFileSync(file_path);
  const json = JSON.parse(data.toString()) as any[];
  const jsBuff = Buffer.from(JSON.stringify(json));

  const result = await (wallet as any).admin_restore(Array.from(jsBuff));
  console.log(`written bytes ${data.length}`);

  console.log({ result });
}

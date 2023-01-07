import fs from 'fs';

export function getEgoConfig<T>(key: string): T {
  const config = JSON.parse(
    fs.readFileSync(`${process.cwd()}/egoconfig.json`, {
      encoding: 'utf-8',
    }),
  );
  return config[key] as T;
}
export const dfxPort = getEgoConfig<number>('dfxPort');
export const isProduction = process.env.NODE_ENV == 'production';

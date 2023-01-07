import fs from 'fs';

interface CredentialProject {
  seedPhrase: string;
  production_pem: string;
  production_cycles_wallet: string;
}

export function getEgoConfig<T>(key: string): T {
  const config = JSON.parse(
    fs.readFileSync(`${process.cwd()}/egoconfig.json`, {
      encoding: 'utf-8',
    }),
  );
  return config[key] as T;
}
export const dfxPort = getEgoConfig<number>('dfxPort');
export const rustEntry = getEgoConfig<string>('rustEntry');
export const artifacts = getEgoConfig<string>('artifacts');
export const productionPem =
  getEgoConfig<CredentialProject>('credentials').production_pem;
export const productionCyclesWallet =
  getEgoConfig<CredentialProject>('credentials').production_cycles_wallet;
export const seedPhrase =
  getEgoConfig<CredentialProject>('credentials').seedPhrase;

export const isProduction = process.env.NODE_ENV === 'production';
export const cyclesCreateCanister = BigInt(
  getEgoConfig<string>('cycles_install_code').replace('_', ''),
);

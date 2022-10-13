/* eslint-disable @typescript-eslint/strict-boolean-expressions */
import { Actor, ActorSubclass, HttpAgent, SignIdentity } from '@dfinity/agent';
import { InterfaceFactory } from '@dfinity/candid/lib/cjs/idl';
import { Principal } from '@dfinity/principal';
import { fetch } from 'cross-fetch';
import { isProduction } from './env';

if (!globalThis.fetch) {
  (globalThis as any).fetch = fetch;
}

if (!global.fetch) {
  (global as any).fetch = fetch;
}

export interface CreateActorResult<T> {
  actor: ActorSubclass<T>;
  agent: HttpAgent;
}

export async function _createActor<T>(
  interfaceFactory: InterfaceFactory,
  canisterId: string,
  identity?: SignIdentity,
  host?: string,
): Promise<CreateActorResult<T>> {
  const agent = new HttpAgent({
    identity,
    host: host ?? !isProduction ? 'http://localhost:8000' : 'https://ic0.app',
  });
  // Only fetch the root key when we're not in prod
  if (!isProduction) {
    await agent.fetchRootKey();
  }

  const actor = Actor.createActor<T>(interfaceFactory, {
    agent,
    canisterId: canisterId === '' ? Principal.fromText('aaaaa-aa') : canisterId,
  });
  return { actor, agent };
}

export async function getActor<T>(
  signIdentity: SignIdentity,
  interfaceFactory: InterfaceFactory,
  canisterId: string,
): Promise<ActorSubclass<T>> {
  const actor = await _createActor<T>(
    interfaceFactory,
    canisterId,
    signIdentity,
  );

  return actor.actor;
}

export async function getActor2<T>(
  signIdentity: SignIdentity,
  interfaceFactory: InterfaceFactory,
  canisterId: string,
): Promise<CreateActorResult<T>> {
  const actor = await _createActor<T>(
    interfaceFactory,
    canisterId,
    signIdentity,
  );

  return actor;
}

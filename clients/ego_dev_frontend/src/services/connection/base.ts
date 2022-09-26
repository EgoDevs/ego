import { Actor, ActorSubclass, HttpAgent, Identity, SignIdentity } from "@dfinity/agent";
import { InterfaceFactory } from "@dfinity/candid/lib/cjs/idl";

export interface CreateActorResult<T> {
  actor: ActorSubclass<T>;
  agent: HttpAgent;
}

export async function _createActor<T>(
  interfaceFactory: InterfaceFactory,
  canisterId: string,
  identity?: Identity,
  host?: string,
): Promise<CreateActorResult<T>> {
  const agent = new HttpAgent({ identity, host: process.env. NODE_ENV !== 'production' ? 'http://localhost:8000' : 'https://ic0.app' });
  // Only fetch the root key when we're not in prod
  // console.log('agent', agent)
  if ( process.env.NODE_ENV !== 'production') {
    // console.log('Fetching root key', process.env.NODE_ENV );
    try {
      await agent.fetchRootKey();
    } catch (err) {
      // console.error('Failed to fetch root key', err);
    }
  }
  // console.log('Fetching canister');
  const actor = Actor.createActor<T>(interfaceFactory, {
    agent,
    canisterId,
  });
  // console.log('Fetching canister11');
  return { actor, agent };
}

export async function getActor<T>(interfaceFactory: InterfaceFactory, canisterId: string, identity?: Identity) {
  return await _createActor<T>(interfaceFactory, canisterId, identity)
}


export interface AbstractConnection<T> {
  identity: Identity;
  actor?: ActorSubclass<T>;
  agent?: HttpAgent;
  canisterId?: string;
}

export class BaseConnection<T> implements AbstractConnection<T> {
  _actor: ActorSubclass<T>;
  _agent: HttpAgent;
  constructor(
    public identity: Identity,
    public canisterId: string,
    public interfaceFactory: InterfaceFactory,
    public actor: ActorSubclass<T>,
    public agent: HttpAgent,
  ) {
    this._actor = actor;
    this._agent = agent;
  }
}
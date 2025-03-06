// @generated file from wasmbuild -- do not edit
// deno-lint-ignore-file
// deno-fmt-ignore-file

export function generate_key_pair(): Ed25519Keypair;
export function ed25519_from_private_key(
  private_key: Uint8Array,
): Ed25519Keypair;
export function ed25519_sign(
  private_key: Uint8Array,
  message: Uint8Array,
): Uint8Array;
export function ed25519_verify(
  public_key: Uint8Array,
  message: Uint8Array,
  signature_buffer: Uint8Array,
): boolean;
export class Ed25519Keypair {
  private constructor();
  free(): void;
}

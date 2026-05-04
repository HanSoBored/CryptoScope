const ENCRYPTION_VERSION = 1;
const PBKDF2_ITERATIONS = 100_000;
const SALT_LENGTH = 16;
const IV_LENGTH = 12;

async function deriveKey(passphrase: string, salt: Uint8Array): Promise<CryptoKey> {
  const encoder = new TextEncoder();
  const keyMaterial = await crypto.subtle.importKey(
    'raw',
    encoder.encode(passphrase),
    'PBKDF2',
    false,
    ['deriveKey']
  );

  return crypto.subtle.deriveKey(
    {
      name: 'PBKDF2',
      salt: salt.buffer as ArrayBuffer,
      iterations: PBKDF2_ITERATIONS,
      hash: 'SHA-256',
    },
    keyMaterial,
    { name: 'AES-GCM', length: 256 },
    false,
    ['encrypt', 'decrypt']
  );
}

export async function encrypt(plaintext: string, passphrase: string): Promise<string> {
  const encoder = new TextEncoder();
  const salt = crypto.getRandomValues(new Uint8Array(SALT_LENGTH));
  const iv = crypto.getRandomValues(new Uint8Array(IV_LENGTH));
  const key = await deriveKey(passphrase, salt);

  const ciphertext = await crypto.subtle.encrypt(
    { name: 'AES-GCM', iv: iv.buffer as ArrayBuffer },
    key,
    encoder.encode(plaintext)
  );

  const packed = new Uint8Array(1 + SALT_LENGTH + IV_LENGTH + ciphertext.byteLength);
  packed[0] = ENCRYPTION_VERSION;
  packed.set(salt, 1);
  packed.set(iv, 1 + SALT_LENGTH);
  packed.set(new Uint8Array(ciphertext), 1 + SALT_LENGTH + IV_LENGTH);

  return btoa(String.fromCharCode(...packed));
}

export async function decrypt(encryptedBase64: string, passphrase: string): Promise<string> {
  const packed = Uint8Array.from(atob(encryptedBase64), (c) => c.charCodeAt(0));
  const version = packed[0];
  if (version !== ENCRYPTION_VERSION) throw new Error(`Unsupported encryption version: ${version}`);

  const salt = packed.slice(1, 1 + SALT_LENGTH);
  const iv = packed.slice(1 + SALT_LENGTH, 1 + SALT_LENGTH + IV_LENGTH);
  const ciphertext = packed.slice(1 + SALT_LENGTH + IV_LENGTH);

  const key = await deriveKey(passphrase, salt);
  const decrypted = await crypto.subtle.decrypt({ name: 'AES-GCM', iv: iv.buffer as ArrayBuffer }, key, ciphertext);

  return new TextDecoder().decode(decrypted);
}

export function generateSessionPassphrase(): string {
  const bytes = crypto.getRandomValues(new Uint8Array(32));
  return Array.from(bytes, (b) => b.toString(16).padStart(2, '0')).join('');
}

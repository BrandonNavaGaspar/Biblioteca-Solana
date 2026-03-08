// Client

console.log("Wallet:", pg.wallet.publicKey.toString());

const balance = await pg.connection.getBalance(pg.wallet.publicKey);
console.log(`Balance: ${balance / web3.LAMPORTS_PER_SOL} SOL`);


// Obtener PDA de la tienda de teléfonos
const [tiendaTelefonosPDA] = web3.PublicKey.findProgramAddressSync(
  [
    Buffer.from("tienda_telefonos"),
    pg.wallet.publicKey.toBuffer()
  ],
  pg.program.programId
);


// Crear tienda de teléfonos
await pg.program.methods
  .crearTiendaTelefonos("Mi Tienda de Celulares")
  .accounts({
    owner: pg.wallet.publicKey,
    tiendaTelefonos: tiendaTelefonosPDA,
    systemProgram: web3.SystemProgram.programId
  })
  .rpc();

console.log("Tienda de teléfonos creada");


// Agregar teléfono
await pg.program.methods
  .agregarTelefono("iPhone 15", "Apple", 25000)
  .accounts({
    owner: pg.wallet.publicKey,
    tiendaTelefonos: tiendaTelefonosPDA
  })
  .rpc();

console.log("Teléfono agregado");


// Ver teléfonos
await pg.program.methods
  .verTelefonos()
  .accounts({
    owner: pg.wallet.publicKey,
    tiendaTelefonos: tiendaTelefonosPDA
  })
  .rpc();

console.log("Teléfonos consultados");

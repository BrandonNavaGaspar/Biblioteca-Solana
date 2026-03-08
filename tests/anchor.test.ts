// No imports needed: web3, anchor, pg and more are globally available

describe("TiendaTelefonos", () => {

  it("Crear tienda", async () => {

    const [tiendaPDA] = await web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("tienda_telefonos"),
        pg.wallet.publicKey.toBuffer()
      ],
      pg.program.programId
    );

    const txHash = await pg.program.methods
      .crearTienda("Mi Tienda de Celulares")
      .accounts({
        owner: pg.wallet.publicKey,
        tienda: tiendaPDA,
        systemProgram: web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Tx:", txHash);

  });


  it("Agregar telefono", async () => {

    const [tiendaPDA] = await web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("tienda_telefonos"),
        pg.wallet.publicKey.toBuffer()
      ],
      pg.program.programId
    );

    const txHash = await pg.program.methods
      .agregarTelefono("iPhone 15", "Apple", 25000)
      .accounts({
        owner: pg.wallet.publicKey,
        tienda: tiendaPDA,
      })
      .rpc();

    console.log("Telefono agregado:", txHash);

  });


  it("Ver telefonos", async () => {

    const [tiendaPDA] = await web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("tienda_telefonos"),
        pg.wallet.publicKey.toBuffer()
      ],
      pg.program.programId
    );

    await pg.program.methods
      .verTelefonos()
      .accounts({
        owner: pg.wallet.publicKey,
        tienda: tiendaPDA,
      })
      .rpc();

    console.log("Telefonos consultados");

  });

});

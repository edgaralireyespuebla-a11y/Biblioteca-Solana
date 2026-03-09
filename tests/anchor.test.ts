// No imports needed: web3, anchor, pg and more are globally available

describe("Farmacia", () => {
  it("crear farmacia", async () => {

    // Generar keypair para la nueva cuenta de farmacia
    const farmaciaKp = new web3.Keypair();

    // Nombre de la farmacia
    const nombre = "Farmacia Solana";

    // Enviar transacción
    const txHash = await pg.program.methods
      .crearFarmacia(nombre)
      .accounts({
        farmacia: farmaciaKp.publicKey,
        owner: pg.wallet.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([farmaciaKp])
      .rpc();

    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);

    // Confirmar transacción
    await pg.connection.confirmTransaction(txHash);

    // Obtener la cuenta creada en la blockchain
    const farmacia = await pg.program.account.farmacia.fetch(
      farmaciaKp.publicKey
    );

    console.log("Datos on-chain:", farmacia);

    // Verificar que el nombre se guardó correctamente
    assert(farmacia.nombre === nombre);
  });
});

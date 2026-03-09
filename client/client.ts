// Cliente - Sistema de compra de medicamentos
console.log("Client wallet:", pg.wallet.publicKey.toString());
const balance = await pg.connection.getBalance(pg.wallet.publicKey);
console.log(`Available balance for buying medicines: ${balance / web3.LAMPORTS_PER_SOL} SOL`);

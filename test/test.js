const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');

const main = async() => {
  const provider = new WsProvider('ws://127.0.0.1:9944');
  //const provider = new HttpProvider('http://localhost:9933');
  const api = await ApiPromise.create({ provider });
  const keyring = new Keyring({ type: 'sr25519' });

  const alice = keyring.addFromUri("//Alice");
  const bob = keyring.addFromUri("//Bob");
  const charlie = keyring.addFromUri("//Charlie");
  const eve = keyring.addFromUri("//Eve");
  const txs = [
    api.tx.balances.transfer(bob.address, 1000000000000),
  ];
  for (i =0; i<100; i++){
    txs.push(api.tx.balances.transfer(bob.address, 1000000000000));
  }

// for (let step = 0; step < 5; step++) {
//     // Runs 5 times, with values of step 0 through 4.
//     console.log('Walking east one step:',step);
//     await api.tx.templateModule.doSomething(10).signAndSend(alice, ({ status }) => {
//         if (status.isInBlock) {
//           console.log(`included in ${status.asInBlock}`);
//         }
//       });
//   }


  //console.log(api.tx);
  await api.tx.utility.batch(txs).signAndSend(alice, ({ status }) => {
    if (status.isInBlock) {
      console.log(`included in ${status.asInBlock}`);
    }
  });


}




main();
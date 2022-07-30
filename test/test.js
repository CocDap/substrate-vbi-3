const {ApiPromise, WsProvider, Keyring} = require('@polkadot/api')


const main= async() => {

    const provider = new WsProvider('ws://127.0.0.1:9944');
    const api = await ApiPromise.create({provider});

    const keyring = new Keyring({type : 'sr25519'});

    const alice = keyring.addFromUri("//Alice");
    const bob = keyring.addFromUri("//Bob");

    const txs = [
        api.tx.balances.transfer(bob.address, 12345),

      ];
    
    for (i =0 ; i<198; i++){
        txs.push(api.tx.balances.transfer(bob.address, 12345));
    }

    await api.tx.utility.batch(txs).signAndSend(alice, ({ status }) => {
    if (status.isInBlock) {
      console.log(`included in ${status.asInBlock}`);
    }
  });

}


main();
const keyManager = require("./key-manager");
const TRANSFER_AMOUNT = process.env.TRANSFER_AMOUNT || 2500000000;

(async function () {
  // In this example, only one key can sign transactions in
  // the name of this account. The key is “account-hash-
  // a1…” under the associated_keys. If you sign the
  // transaction using “account-hash-a1…”, the signed
  // transaction will have a weight equal to 1. For
  // deployments or key management, the weight required is also
  // 1. Therefore, the associated key meets the deployment and
  // key management thresholds and can perform both actions.

  // To achive the task, we will:
  // 1. Set mainAccount's weight to 1.
  // 2. Set Keys Management Threshold to 1.
  // 3. Set Deploy Threshold to 1.
  // 4. Add new key with weight 1 (from main account).

  let deploy;

  // 0. Initial state of the account.
  // There should be only one associated key (fuacet) with weight 1.
  // Deployment Threshold should be set to 1.
  // Key Management Threshold should be set to 1.
  let masterKey = keyManager.randomMasterKey();
  let mainAccount = masterKey.deriveIndex(1);

  console.log("\n0.1 Fund main account.\n");
  await keyManager.fundAccount(mainAccount);
  await keyManager.printAccount(mainAccount);

  console.log("\n[x]0.2 Install Keys Manager contract");
  deploy = keyManager.keys.buildContractInstallDeploy(mainAccount);
  await keyManager.sendDeploy(deploy, [mainAccount]);
  await keyManager.printAccount(mainAccount);

  // 1. Set mainAccount's weight to 1
  console.log("\n1. Set faucet's weight to 1\n");
  deploy = keyManager.keys.setKeyWeightDeploy(mainAccount, mainAccount, 1);
  await keyManager.sendDeploy(deploy, [mainAccount]);
  await keyManager.printAccount(mainAccount);

  // 2. Set Keys Management Threshold to 1.
  console.log("\n2. Set Keys Management Threshold to 1\n");
  deploy = keyManager.keys.setKeyManagementThresholdDeploy(mainAccount, 1);
  await keyManager.sendDeploy(deploy, [mainAccount]);
  await keyManager.printAccount(mainAccount);

  // 3. Set Deploy Threshold to 1.
  console.log("\n3. Set Deploy Threshold to 1.\n");
  deploy = keyManager.keys.setDeploymentThresholdDeploy(mainAccount, 1);
  await keyManager.sendDeploy(deploy, [mainAccount]);
  await keyManager.printAccount(mainAccount);

  // 4. Add new key with weight 1.
  console.log("\n4. Add new key with weight 1.\n");
  deploy = keyManager.keys.setKeyWeightDeploy(mainAccount, mainAccount, 1);
  await keyManager.sendDeploy(deploy, [mainAccount]);
  await keyManager.printAccount(mainAccount);
})();

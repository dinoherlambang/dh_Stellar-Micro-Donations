import { Server, Networks, TransactionBuilder, Operation, Asset, Keypair } from 'stellar-sdk';

export class StellarService {
  private server: Server;
  private contractId: string;
  private account: Keypair;

  constructor(horizonUrl: string, contractId: string, secretKey: string) {
    this.server = new Server(horizonUrl);
    this.contractId = contractId;
    this.account = Keypair.fromSecret(secretKey);
  }

  private async signAndSubmitTransaction(transaction: TransactionBuilder) {
    const builtTx = transaction.build();
    builtTx.sign(this.account);
    return await this.server.submitTransaction(builtTx);
  }

  async createProject(name: string, goal: number) {
    const transaction = new TransactionBuilder(this.account.publicKey(), {
      fee: await this.server.fetchBaseFee(),
      networkPassphrase: Networks.TESTNET,
    })
    .addOperation(Operation.invokeHostFunction({
      function: 'create_project',
      parameters: [name, goal.toString()],
      contractId: this.contractId,
    }))
    .setTimeout(30);

    return await this.signAndSubmitTransaction(transaction);
  }

  async donate(project: string, amount: number) {
    const transaction = new TransactionBuilder(this.account.publicKey(), {
      fee: await this.server.fetchBaseFee(),
      networkPassphrase: Networks.TESTNET,
    })
    .addOperation(Operation.invokeHostFunction({
      function: 'donate',
      parameters: [project, amount.toString()],
      contractId: this.contractId,
    }))
    .setTimeout(30);

    return await this.signAndSubmitTransaction(transaction);
  }

  async getProjects() {
    // This is a placeholder implementation. In a real-world scenario,
    // you would need to implement a method in the smart contract to return all projects.
    const transaction = new TransactionBuilder(this.account.publicKey(), {
      fee: await this.server.fetchBaseFee(),
      networkPassphrase: Networks.TESTNET,
    })
    .addOperation(Operation.invokeHostFunction({
      function: 'get_all_projects', // Assuming this function exists in the smart contract
      parameters: [],
      contractId: this.contractId,
    }))
    .setTimeout(30);

    const result = await this.signAndSubmitTransaction(transaction);
    // Parse the result to extract project information
    // This will depend on how the smart contract returns the data
    return result;
  }

  async getProjectStatus(project: string) {
    const transaction = new TransactionBuilder(this.account.publicKey(), {
      fee: await this.server.fetchBaseFee(),
      networkPassphrase: Networks.TESTNET,
    })
    .addOperation(Operation.invokeHostFunction({
      function: 'get_project_status',
      parameters: [project],
      contractId: this.contractId,
    }))
    .setTimeout(30);

    return await this.signAndSubmitTransaction(transaction);
  }
}
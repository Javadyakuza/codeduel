import { expect } from "chai";
import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { AnchorProvider } from "@project-serum/anchor";
import { Program, Provider } from "@project-serum/anchor";

// Replace these with your actual program ID and wallet
const PROGRAM_ID = new PublicKey("YOUR_PROGRAM_ID_HERE");
const WALLET_KEYPAIR = Keypair.generate();

describe("code-duel", () => {
  let provider: Provider;
  let program: Program;
  let prizePool: PublicKey;
  let questions: PublicKey;
  let rivals: PublicKey;
  let rivalsBalances: PublicKey;

  before(async () => {
    provider = new AnchorProvider(new Connection("http://localhost:8899"), WALLET_KEYPAIR);
    program = new Program(codeDuel, PROGRAM_ID, provider);

    const [prizePoolAccount, tx] = await program.methods.createPrizePool().rpc();
    prizePool = prizePoolAccount.publicKey;

    const [questionsAccount, tx] = await program.methods.createQuestions().rpc();
    questions = questionsAccount.publicKey;

    const [rivalsAccount, tx] = await program.methods.createRivals().rpc();
    rivals = rivalsAccount.publicKey;

    const [rivalsBalancesAccount, tx] = await program.methods.createRivalsBalances().rpc();
    rivalsBalances = rivalsBalancesAccount.publicKey;
  });

  it("should deposit funds to the prize pool", async () => {
    const initialPoolBalance = await program.account.prizePool.fetch(prizePool);
    const depositAmount = 100_000_000; // 1 SOL

    await program.methods.deposit(depositAmount).rpc();

    const updatedPoolBalance = await program.account.prizePool.fetch(prizePool);

    expect(updatedPoolBalance.totalPool).to.equal(initialPoolBalance.totalPool + depositAmount);
  });

  it("should create a new question", async () => {
    const question = {
      id: 1,
      name: "Who can write the best haiku?",
      status: QuestionStatus.Open,
      reward: 10_000_000, // 0.1 SOL
      rivalId: 2,
      daredevilId: 3,
      currentPrizePool: 0,
      entranceFee: 5_000_000, // 0.05 SOL
      deadline: Timestamp.now().plus(Duration.fromHours(24)),
    };

    await program.methods.openQuestion(question).rpc();

    const questionsAccount = await program.account.questions.fetch(questions);
    const createdQuestion = questionsAccount.entries[question.id];

    expect(createdQuestion).to.deep.equal(question);
  });

  // Add more tests for other functionalities like closing a question, etc.

  // Remember to replace and customize these tests based on your specific logic and functions.
});
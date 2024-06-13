/* Autogenerated file. Do not edit manually. */
/* tslint:disable */
/* eslint-disable */
import type {
  BaseContract,
  BigNumberish,
  BytesLike,
  FunctionFragment,
  Result,
  Interface,
  EventFragment,
  AddressLike,
  ContractRunner,
  ContractMethod,
  Listener,
} from "ethers";
import type {
  TypedContractEvent,
  TypedDeferredTopicFilter,
  TypedEventLog,
  TypedLogDescription,
  TypedListener,
  TypedContractMethod,
} from "../common";

export interface PayCommitmentInterface extends Interface {
  getFunction(
    nameOrSignature:
      | "depositERC20"
      | "groth16VerifierAddress"
      | "nullifierHashes"
      | "tokenAddress"
      | "withdraw"
  ): FunctionFragment;

  getEvent(nameOrSignatureOrTopic: "LogClaim" | "LogDeposit"): EventFragment;

  encodeFunctionData(
    functionFragment: "depositERC20",
    values: [BigNumberish, BytesLike]
  ): string;
  encodeFunctionData(
    functionFragment: "groth16VerifierAddress",
    values?: undefined
  ): string;
  encodeFunctionData(
    functionFragment: "nullifierHashes",
    values: [BytesLike]
  ): string;
  encodeFunctionData(
    functionFragment: "tokenAddress",
    values?: undefined
  ): string;
  encodeFunctionData(
    functionFragment: "withdraw",
    values: [
      [BigNumberish, BigNumberish],
      [[BigNumberish, BigNumberish], [BigNumberish, BigNumberish]],
      [BigNumberish, BigNumberish],
      [BigNumberish, BigNumberish, BigNumberish, BigNumberish]
    ]
  ): string;

  decodeFunctionResult(
    functionFragment: "depositERC20",
    data: BytesLike
  ): Result;
  decodeFunctionResult(
    functionFragment: "groth16VerifierAddress",
    data: BytesLike
  ): Result;
  decodeFunctionResult(
    functionFragment: "nullifierHashes",
    data: BytesLike
  ): Result;
  decodeFunctionResult(
    functionFragment: "tokenAddress",
    data: BytesLike
  ): Result;
  decodeFunctionResult(functionFragment: "withdraw", data: BytesLike): Result;
}

export namespace LogClaimEvent {
  export type InputTuple = [
    receipient: AddressLike,
    amount: BigNumberish,
    commitment: BytesLike,
    nullifierHash: BytesLike
  ];
  export type OutputTuple = [
    receipient: string,
    amount: bigint,
    commitment: string,
    nullifierHash: string
  ];
  export interface OutputObject {
    receipient: string;
    amount: bigint;
    commitment: string;
    nullifierHash: string;
  }
  export type Event = TypedContractEvent<InputTuple, OutputTuple, OutputObject>;
  export type Filter = TypedDeferredTopicFilter<Event>;
  export type Log = TypedEventLog<Event>;
  export type LogDescription = TypedLogDescription<Event>;
}

export namespace LogDepositEvent {
  export type InputTuple = [
    depositor: AddressLike,
    amount: BigNumberish,
    commitment: BytesLike
  ];
  export type OutputTuple = [
    depositor: string,
    amount: bigint,
    commitment: string
  ];
  export interface OutputObject {
    depositor: string;
    amount: bigint;
    commitment: string;
  }
  export type Event = TypedContractEvent<InputTuple, OutputTuple, OutputObject>;
  export type Filter = TypedDeferredTopicFilter<Event>;
  export type Log = TypedEventLog<Event>;
  export type LogDescription = TypedLogDescription<Event>;
}

export interface PayCommitment extends BaseContract {
  connect(runner?: ContractRunner | null): PayCommitment;
  waitForDeployment(): Promise<this>;

  interface: PayCommitmentInterface;

  queryFilter<TCEvent extends TypedContractEvent>(
    event: TCEvent,
    fromBlockOrBlockhash?: string | number | undefined,
    toBlock?: string | number | undefined
  ): Promise<Array<TypedEventLog<TCEvent>>>;
  queryFilter<TCEvent extends TypedContractEvent>(
    filter: TypedDeferredTopicFilter<TCEvent>,
    fromBlockOrBlockhash?: string | number | undefined,
    toBlock?: string | number | undefined
  ): Promise<Array<TypedEventLog<TCEvent>>>;

  on<TCEvent extends TypedContractEvent>(
    event: TCEvent,
    listener: TypedListener<TCEvent>
  ): Promise<this>;
  on<TCEvent extends TypedContractEvent>(
    filter: TypedDeferredTopicFilter<TCEvent>,
    listener: TypedListener<TCEvent>
  ): Promise<this>;

  once<TCEvent extends TypedContractEvent>(
    event: TCEvent,
    listener: TypedListener<TCEvent>
  ): Promise<this>;
  once<TCEvent extends TypedContractEvent>(
    filter: TypedDeferredTopicFilter<TCEvent>,
    listener: TypedListener<TCEvent>
  ): Promise<this>;

  listeners<TCEvent extends TypedContractEvent>(
    event: TCEvent
  ): Promise<Array<TypedListener<TCEvent>>>;
  listeners(eventName?: string): Promise<Array<Listener>>;
  removeAllListeners<TCEvent extends TypedContractEvent>(
    event?: TCEvent
  ): Promise<this>;

  depositERC20: TypedContractMethod<
    [_amount: BigNumberish, _commitment: BytesLike],
    [void],
    "payable"
  >;

  groth16VerifierAddress: TypedContractMethod<[], [string], "view">;

  nullifierHashes: TypedContractMethod<[arg0: BytesLike], [boolean], "view">;

  tokenAddress: TypedContractMethod<[], [string], "view">;

  withdraw: TypedContractMethod<
    [
      _pA: [BigNumberish, BigNumberish],
      _pB: [[BigNumberish, BigNumberish], [BigNumberish, BigNumberish]],
      _pC: [BigNumberish, BigNumberish],
      _pubSignals: [BigNumberish, BigNumberish, BigNumberish, BigNumberish]
    ],
    [void],
    "nonpayable"
  >;

  getFunction<T extends ContractMethod = ContractMethod>(
    key: string | FunctionFragment
  ): T;

  getFunction(
    nameOrSignature: "depositERC20"
  ): TypedContractMethod<
    [_amount: BigNumberish, _commitment: BytesLike],
    [void],
    "payable"
  >;
  getFunction(
    nameOrSignature: "groth16VerifierAddress"
  ): TypedContractMethod<[], [string], "view">;
  getFunction(
    nameOrSignature: "nullifierHashes"
  ): TypedContractMethod<[arg0: BytesLike], [boolean], "view">;
  getFunction(
    nameOrSignature: "tokenAddress"
  ): TypedContractMethod<[], [string], "view">;
  getFunction(
    nameOrSignature: "withdraw"
  ): TypedContractMethod<
    [
      _pA: [BigNumberish, BigNumberish],
      _pB: [[BigNumberish, BigNumberish], [BigNumberish, BigNumberish]],
      _pC: [BigNumberish, BigNumberish],
      _pubSignals: [BigNumberish, BigNumberish, BigNumberish, BigNumberish]
    ],
    [void],
    "nonpayable"
  >;

  getEvent(
    key: "LogClaim"
  ): TypedContractEvent<
    LogClaimEvent.InputTuple,
    LogClaimEvent.OutputTuple,
    LogClaimEvent.OutputObject
  >;
  getEvent(
    key: "LogDeposit"
  ): TypedContractEvent<
    LogDepositEvent.InputTuple,
    LogDepositEvent.OutputTuple,
    LogDepositEvent.OutputObject
  >;

  filters: {
    "LogClaim(address,uint256,bytes32,bytes32)": TypedContractEvent<
      LogClaimEvent.InputTuple,
      LogClaimEvent.OutputTuple,
      LogClaimEvent.OutputObject
    >;
    LogClaim: TypedContractEvent<
      LogClaimEvent.InputTuple,
      LogClaimEvent.OutputTuple,
      LogClaimEvent.OutputObject
    >;

    "LogDeposit(address,uint256,bytes32)": TypedContractEvent<
      LogDepositEvent.InputTuple,
      LogDepositEvent.OutputTuple,
      LogDepositEvent.OutputObject
    >;
    LogDeposit: TypedContractEvent<
      LogDepositEvent.InputTuple,
      LogDepositEvent.OutputTuple,
      LogDepositEvent.OutputObject
    >;
  };
}

import React, { useContext } from "react";
import { Vector3 } from "three";

export class ArmVertices {
  public constructor(public readonly inner: Vector3[]) {}

  public get end_effector(): Vector3 {
    return this.inner[this.inner.length - 1];
  }

  public static deserialize(serialized: number[][]): ArmVertices {
    return new ArmVertices(
      serialized.map((vec: number[]) => new Vector3(vec[0], vec[1], vec[2]))
    );
  }
}

export class ArmAngles {
  public constructor(public readonly inner: number[]) {}

  public static deserialize(serialized: number[]): ArmAngles {
    return new ArmAngles(serialized);
  }
}

export interface ISerializedArmState {
  vertices: number[][];
  angles: number[];
}

export class ArmState {
  public constructor(
    public readonly vertices: ArmVertices,
    public readonly angles: ArmAngles
  ) {}

  public static deserialize(serialized: ISerializedArmState): ArmState {
    return new ArmState(
      ArmVertices.deserialize(serialized.vertices),
      ArmAngles.deserialize(serialized.angles)
    );
  }
}

export interface IMyArmContext {
  state: ArmState;
}

export const MyArmContext = React.createContext<IMyArmContext | null>(null);

export const useArmContext = (): IMyArmContext => {
  const context = useContext(MyArmContext);

  if (!context) {
    throw new Error(
      "useArmContext must be used within an MyArmContext.Provider"
    );
  }

  return context;
};

export interface IMyArmProviderProps {
  children: React.ReactNode;
}

export const MyArmProvider = ({
  children,
}: IMyArmProviderProps): React.ReactElement => {
  const [state, setState] = React.useState<ArmState>(
    new ArmState(
      new ArmVertices([
        new Vector3(0, 0, 0),
        new Vector3(0, 10, 0),
        new Vector3(0, 20, 0),
        new Vector3(0, 3, 0),
        new Vector3(0, 40, 0),
        new Vector3(0, 50, 0),
      ]),
      new ArmAngles([1, 1, 1, 1, 1])
    )
  );

  return (
    <MyArmContext.Provider value={{ state }}>{children}</MyArmContext.Provider>
  );
};

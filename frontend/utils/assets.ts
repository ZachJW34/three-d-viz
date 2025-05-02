import { v4 } from "uuid";

export class TRANFORM {
  static IDENTITY(): Transform {
    return { trans: [0.0, 0.0, 0.0], rot: [0.0, 0.0, 0.0, 1.0] };
  }
}

export type Transform = {
  trans: number[];
  rot: number[];
};

export type TransformStartEnd = {
  start: Transform;
  end: Transform;
};

export type TransformExpr = {
  t_exprs: string[];
  r_exprs: string[];
};

export const genDefaultTransformStartEnd = (): TransformStartEnd => ({
  start: TRANFORM.IDENTITY(),
  end: TRANFORM.IDENTITY(),
});

export const genDefaultTransformExpr = (): TransformExpr => ({
  t_exprs: ["0", "0", "0"],
  r_exprs: ["0", "0", "0", "1"],
});

type Animator =
  | { type: "StartEnd"; value: TransformStartEnd }
  | { type: "Expr"; value: TransformExpr };

const CUBE = "cube" as const;
const SPHERE = "sphere" as const;
const TORUS = "torus" as const;
const CONE = "cone" as const;
const CYLINDER = "cylinder" as const;

export const PrimitiveGenerators = {
  [CUBE]: {
    type: CUBE,
    x: 1,
    y: 1,
    z: 1,
  },
  [SPHERE]: { type: SPHERE, radius: 1 },
  [TORUS]: {
    type: TORUS,
    inner_radius: 0.5,
    outer_radius: 1,
  },
  [CONE]: {
    type: CONE,
    radius: 0.5,
    height: 1,
  },
  [CYLINDER]: {
    type: CYLINDER,
    radius: 0.5,
    height: 1,
  },
};

export type PrimitiveKey = keyof typeof PrimitiveGenerators;
export type Primitive =
  (typeof PrimitiveGenerators)[keyof typeof PrimitiveGenerators];

export function genPrimitive(key: PrimitiveKey): Primitive {
  return structuredClone(PrimitiveGenerators[key]);
}

export type Asset = {
  id: string;
  name: string;
  primitive: Primitive;
  animator: Animator;
  children: Asset[];
};

export type State = {
  assets: Asset[];
  version: String;
};

export const genDefaultAsset = (type: PrimitiveKey = "cube"): Asset => ({
  id: v4(),
  name: capitalize(type),
  primitive: genPrimitive(type),
  animator: {
    type: "StartEnd",
    value: genDefaultTransformStartEnd(),
  },
  children: [],
});

const STATE_KEY = "state";

export function initState() {
  if (!localStorage.getItem(STATE_KEY)) {
    const state: State = {
      assets: [genDefaultAsset()],
      version: v4(),
    };
    localStorage.setItem(STATE_KEY, JSON.stringify(state));
  }
}

export function saveStateLS(state: State) {
  localStorage.setItem(STATE_KEY, JSON.stringify(state));
}

export function getStateLS(): State {
  return JSON.parse(localStorage.getItem(STATE_KEY) || "");
}

export function bevyUpdateState(state: State) {
  if (window.wasmBindings) {
    window.wasmBindings.update_state(
      JSON.stringify({ ...state, version: v4() })
    );
  }
}

export function genChild(siblings: Asset[]) {
  const asset = genDefaultAsset();
  asset.name = generateSafeName(asset.primitive.type, siblings);
  return asset;
}

export function generateSafeName(
  type: PrimitiveKey,
  siblings: Asset[]
): string {
  const genSafe = () => {
    if (siblings.length === 0) {
      return type;
    }
    const uniqueNames = new Set(siblings.map((s) => s.name.toLowerCase()));

    console.log({ uniqueNames });

    for (let i = 0; i < uniqueNames.size; i++) {
      let nextName = i === 0 ? type : `${type} (${i})`;
      console.log({ nextName });
      if (!uniqueNames.has(nextName)) {
        return nextName;
      }
    }

    return `${type} (${uniqueNames.size})`;
  };

  return capitalize(genSafe());
}

function capitalize(str: string) {
  return str.charAt(0).toUpperCase() + str.slice(1);
}

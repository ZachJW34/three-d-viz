import { v4 } from "uuid";
import {
  genDefaultAsset,
  genDefaultTransformStartEnd,
  PrimitiveGenerators,
  TRANFORM,
  type Asset,
  type Transform,
} from "./assets";

export function generateSpherePoints(n: number, radius = 1): Asset {
  const primitiveRadius = 0.05;
  const parent = genDefaultAsset("sphere");
  if (parent.primitive.type !== "sphere") {
    throw Error("todo clean up type assertions");
  }
  parent.primitive.radius = primitiveRadius;
  const offset = 2 / n;
  const increment = Math.PI * (3 - Math.sqrt(5)); // golden angle

  for (let i = 0; i < n; i++) {
    const y = i * offset - 1 + offset / 2;
    const r = Math.sqrt(1 - y * y);
    const phi = i * increment;

    const x = Math.cos(phi) * r;
    const z = Math.sin(phi) * r;
    let transform: Transform = {
      trans: [x * radius, y * radius, z * radius],
      rot: TRANFORM.IDENTITY().rot,
    };
    const primitive = structuredClone(PrimitiveGenerators["sphere"]);
    primitive.radius = primitiveRadius;
    const animator = genDefaultTransformStartEnd();
    animator.start = transform;
    animator.end = transform;

    parent.children.push({
      id: v4(),
      name: `Sphere ${i}`,
      primitive,
      animator: { type: "StartEnd", value: animator },
      children: [],
    });
  }

  return parent;
}

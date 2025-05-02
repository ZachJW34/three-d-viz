/// <reference types="vite/client" />
interface Window {
  wasmBindings: {
    // set_transformation(
    //   start_trans: number[],
    //   start_rot: number[],
    //   end_rot: number[],
    //   end_rot: number[]
    // ): void;
    update_state(state: string): void;
  };
}

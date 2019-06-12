
export enum actionType {
  ROTATE = "rotate",
}

interface RotateAction {
  type: actionType.ROTATE;
  payload: boolean;
}

export const startAction: RotateAction = {
  type: actionType.ROTATE,
  payload: true,
};

export const stopAction: RotateAction = {
  type: actionType.ROTATE,
  payload: false,
};

export type action = RotateAction;

export interface RotateState {
  rotating: boolean;
}

export const initialState = {
  rotating: true,
};

export default function rorateReducer(state: RotateState = initialState, action: action): RotateState {
  switch (action.type) {
    case actionType.ROTATE:
      return {
        rotating: action.payload,
      };

    default:
      return state;
  };
}

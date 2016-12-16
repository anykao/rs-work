import { routerRedux, Router, RouteProps } from 'dva/router';

interface IAction {
  type: string
  payload: any
}

const delay = (timeout) => new Promise(resolve => setTimeout(resolve, timeout));

export default {
  namespace: 'count',
  state: 0,
  reducers: {
    add(state: number) { return state + 1; },
    minus(state: number) { return state - 1; },
  },
  effects: {
    *addWithDelay(action: IAction, { call, put }) {
      yield call(delay, 500);
      yield put({ type: 'add' });
    },
    *redirect(action: IAction, { put }) {
      yield put(routerRedux.push('/abc'));
    },
  },
}

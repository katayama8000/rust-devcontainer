import { InjectionKey } from "vue";
import { createStore, Store } from "vuex";

// ストアのステートに対して型を定義
export interface State {
  count: number;
}

// インジェクションキーを定義
export const key: InjectionKey<Store<State>> = Symbol();

export const store = createStore<State>({
  state() {
    return {
      count: 1,
    };
  },
});

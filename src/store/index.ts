import { InjectionKey } from "vue";
import { createStore, Store } from "vuex";

// ストアのステートに対して型を定義します
export interface State {
  leftItems: string[];
  rightItems: string[];
}

// インジェクションキーを定義します
export const key: InjectionKey<Store<State>> = Symbol();

export const store = createStore<State>({
  state: {
    leftItems: ["apple", "banana", "orange"],
    rightItems: ["grape", "pear", "watermelon"],
  },
  getters: {},
  mutations: {},
  actions: {},
  modules: {},
});

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
  mutations: {
    moveLeftToRight(state) {
      if (state.leftItems.length === 0) {
        alert("左側にアイテムがありません");
        return;
      }
      const item: string = state.leftItems.slice(-1)[0];
      state.rightItems = [...state.rightItems, item];
      state.leftItems = state.leftItems.slice(0, -1);
    },

    moveRightToLeft(state) {
      if (state.rightItems.length === 0) {
        alert("右のリストに要素がありません");
        return;
      }
      const item: string = state.rightItems.slice(-1)[0];
      state.leftItems = [...state.leftItems, item];
      state.rightItems = state.rightItems.slice(0, -1);
      console.log(state.rightItems);
    },
  },
  actions: {},
  modules: {},
});

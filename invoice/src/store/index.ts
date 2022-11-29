import { InjectionKey } from "vue";
import { createStore, Store } from "vuex";
import { db } from "../main";
import { collection, getDocs } from "firebase/firestore";
import { doc, deleteDoc } from "firebase/firestore";
import { updateDoc } from "firebase/firestore";
import router from "../router/index";
import { getAuth } from "firebase/auth";
import { signInWithEmailAndPassword } from "firebase/auth";
import { createUserWithEmailAndPassword } from "firebase/auth";
import type { invoiceType } from "@/types/invoice.model";

// ストアのステートに対して型を定義
export interface State {
  invoiceData: invoiceType[];
  invoiceModal: boolean;
  modalActive: boolean;
  invoicesLoaded: boolean;
  currentInvoiceArray: invoiceType[];
  editInvoice: boolean;
  idToken: string | null;
}

// インジェクションキーを定義
export const key: InjectionKey<Store<State>> = Symbol();

export const store = createStore<State>({
  state() {
    return {
      invoiceData: [],
      invoiceModal: false,
      modalActive: false,
      invoicesLoaded: false,
      currentInvoiceArray: [],
      editInvoice: false,
      idToken: null,
    };
  },

  getters: {
    idToken: (state) => state.idToken,
    getInvoiceData: (state) => state.invoiceData,
  },
  mutations: {
    TOGGLE_INVOICE(state) {
      state.invoiceModal = !state.invoiceModal;
    },
    TOGGLE_MODAL(state) {
      state.modalActive = !state.modalActive;
    },
    SET_INVOICE_DATA(state, payload: invoiceType) {
      state.invoiceData.push(payload);
    },
    INVOICES_LOADED(state) {
      state.invoicesLoaded = true;
    },
    SET_CURRENT_INVOICE(state, payload: invoiceType["invoiceId"]) {
      state.currentInvoiceArray = state.invoiceData.filter((invoice) => {
        return invoice.invoiceId === payload;
      });
    },
    TOGGLE_EDIT_INVOICE(state) {
      state.editInvoice = !state.editInvoice;
    },
    DELETE_INVOICE(state, payload: invoiceType["docId"]) {
      //削除されたデータを抜いて新しいstateを作成
      state.invoiceData = state.invoiceData.filter(
        (invoice) => invoice.docId !== payload
      );
    },
    UPDATE_STATUS_TO_PAID(state, payload: invoiceType["docId"]) {
      state.invoiceData.forEach((invoice) => {
        if (invoice.docId === payload) {
          invoice.invoicePaid = true;
          invoice.invoicePending = false;
        }
      });
    },
    UPDATE_STATUS_TO_PENDING(state, payload: invoiceType["docId"]) {
      state.invoiceData.forEach((invoice) => {
        if (invoice.docId === payload) {
          invoice.invoicePaid = false;
          invoice.invoicePending = true;
          invoice.invoiceDraft = false;
        }
      });
    },

    upDataIdToken(state, idToken: string) {
      state.idToken = idToken;
    },
  },
  actions: {
    async GET_INVOICES({ commit, state }) {
      const results = await getDocs(collection(db, "invoice"));
      results.forEach((doc) => {
        if (!state.invoiceData.some((invoice) => invoice.docId === doc.id)) {
          const data: invoiceType = {
            docId: doc.id,
            invoiceId: doc.data().invoiceId,
            billerStreetAddress: doc.data().billerStreetAddress,
            billerCity: doc.data().billerCity,
            billerZipCode: doc.data().billerZipCode,
            billerCountry: doc.data().billerCountry,
            clientName: doc.data().clientName,
            clientEmail: doc.data().clientEmail,
            clientStreetAddress: doc.data().clientStreetAddress,
            clientCity: doc.data().clientCity,
            clientZipCode: doc.data().clientZipCode,
            clientCountry: doc.data().clientCountry,
            invoiceDateUnix: doc.data().invoiceDateUnix,
            invoiceDate: doc.data().invoiceDate,
            paymentTerms: doc.data().paymentTerms,
            paymentDueDateUnix: doc.data().paymentDueDateUnix,
            paymentDueDate: doc.data().paymentDueDate,
            productDescription: doc.data().productDescription,
            invoiceItemList: doc.data().invoiceItemList,
            invoiceTotal: doc.data().invoiceTotal,
            invoicePending: doc.data().invoicePending,
            invoiceDraft: doc.data().invoiceDraft,
            invoicePaid: doc.data().invoicePaid,
          };
          console.log(data, "this is data");
          commit("SET_INVOICE_DATA", data);
        }
      });
      commit("INVOICES_LOADED");
    },

    async UPDATE_INVOICE(
      { commit, dispatch },
      { docId, routeId }: { docId: invoiceType["docId"]; routeId: string }
    ): Promise<void> {
      commit("DELETE_INVOICE", docId);
      await dispatch("GET_INVOICES");
      commit("TOGGLE_INVOICE");
      commit("TOGGLE_EDIT_INVOICE");
      commit("SET_CURRENT_INVOICE", routeId);
    },

    async DELETE_INVOICE({ commit }, docId: invoiceType["docId"]) {
      await deleteDoc(doc(db, "invoice", docId));
      commit("DELETE_INVOICE", docId);
    },

    async UPDATE_STATUS_TO_PAID({ commit }, docId: invoiceType["docId"]) {
      const getInvoice = doc(db, "invoice", docId);
      await updateDoc(getInvoice, {
        invoicePaid: true,
        invoicePending: false,
      });
      commit("UPDATE_STATUS_TO_PAID", docId);
    },

    async UPDATE_STATUS_TO_PENDING({ commit }, docId: invoiceType["docId"]) {
      const getInvoice = doc(db, "invoice", docId);
      await updateDoc(getInvoice, {
        invoicePaid: false,
        invoicePending: true,
        invoiceDraft: false,
      });
      commit("UPDATE_STATUS_TO_PENDING", docId);
    },

    async signin({ commit }, authData: { email: string; password: string }) {
      const auth = getAuth();
      signInWithEmailAndPassword(auth, authData.email, authData.password)
        .then((userCredential) => {
          const user = userCredential.user;
          alert("ログイン成功");
          commit("upDataIdToken", user.uid);
          router.push("/");
        })
        .catch((error) => {
          const errorCode = error.code;
          alert(errorCode);
        });
    },

    signup({ commit }, authData: { email: string; password: string }) {
      const auth = getAuth();
      createUserWithEmailAndPassword(auth, authData.email, authData.password)
        .then((userCredential) => {
          const user = userCredential.user;
          alert("新規登録完了");
          commit("upDataIdToken", user.uid);
          router.push("/");
        })
        .catch((error) => {
          const errorCode = error.code;
          alert("エラー\n新しいメールアドレスorパスワードを\n入力してください");
        });
    },

    logout({ commit }) {
      commit("upDataIdToken", null);
      router.replace("/signin");
    },

    test() {
      console.log("test");
    },
  },
});

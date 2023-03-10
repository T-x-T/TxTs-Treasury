export const state = () => ({
	accounts: [],
	currencies: [],
	recipients: [],
	transactions: [],
	tags: [],
	assets: []
});

export const mutations = {
	accounts(state, payload) {
		state.accounts = payload;
	},

	currencies(state, payload) {
		state.currencies = payload;
	},

	recipients(state, payload) {
		state.recipients = payload;
	},

	transactions(state, payload) {
		state.transactions = payload;
	},

	tags(state, payload) {
		state.tags = payload;
	},

	assets(state, payload) {
		state.assets = payload;
	}
}

export const actions = {
	async fetchAll(context) {
		context.commit("accounts", (await this.$axios.$get("/api/v1/accounts/all")).sort((a, b) => a.id - b.id));
		context.commit("currencies", (await this.$axios.$get("/api/v1/currencies/all")).sort((a, b) => a.id - b.id));
		context.commit("recipients", (await this.$axios.$get("/api/v1/recipients/all")).sort((a, b) => a.id - b.id));
		context.commit("transactions", (await this.$axios.$get("/api/v1/transactions/all")).sort((a, b) => a.id - b.id));
		context.commit("tags", (await this.$axios.$get("/api/v1/tags/all")).sort((a, b) => a.id - b.id));
		context.commit("assets", (await this.$axios.$get("/api/v1/assets/all")).sort((a, b) => a.id - b.id));
	},

	async fetchAccounts(context) {
		try {
			context.commit("accounts", (await this.$axios.$get("/api/v1/accounts/all")).sort((a, b) => a.id - b.id));
		} catch(e) {
			console.log(e.response)
		}
	},

	async fetchCurrencies(context) {
		try {
			context.commit("currencies", (await this.$axios.$get("/api/v1/currencies/all")).sort((a, b) => a.id - b.id));
		} catch(e) {
			console.log(e.response)
		}
	},

	async fetchRecipients(context) {
		try {
			context.commit("recipients", (await this.$axios.$get("/api/v1/recipients/all")).sort((a, b) => a.id - b.id));
		} catch(e) {
			console.log(e.response)
		}
	},

	async fetchTransactions(context) {
		try {
			context.commit("transactions", (await this.$axios.$get("/api/v1/transactions/all")).sort((a, b) => a.id - b.id));
		} catch(e) {
			console.log(e.response)
		}
	},

	async fetchTags(context) {
		try {
			context.commit("tags", (await this.$axios.$get("/api/v1/tags/all")).sort((a, b) => a.id - b.id));
		} catch(e) {
			console.log(e.response)
		}
	},

	async fetchAssets(context) {
		try {
			context.commit("assets", (await this.$axios.$get("/api/v1/assets/all")).sort((a, b) => a.id - b.id));
		} catch(e) {
			console.log(e.response)
		}
	}
}
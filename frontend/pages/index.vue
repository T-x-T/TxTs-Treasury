<template>
	<div>
		<h2>This is the Dashboard</h2>
		<div id="grid" v-if="show">
			<div class="gridItem small" v-for="(amount, currency_id, i) in total_per_currency" :key="i">
				<TotalBalance :currency_id="parseInt(currency_id)" :amount="amount"/>	
			</div>
			<div class="gridItem medium">
				<CustomPieChart
					type="recipients"
					api_path="/api/v1/reports/spending_per_recipient_in_date_range"
					label_property="name"
				/>
			</div>
			<div class="gridItem medium">
				<CustomPieChart
					type="tags"
					api_path="/api/v1/reports/spending_per_tag_in_date_range"
					label_property="name"
					:showOnlyParentsToggle="true"
				/>
			</div>
			<div class="gridItem large">
				<CustomLineChart
					api_path="/api/v1/reports/balance_over_time"
					label_property="symbol"
					:aggregated="true"
					title="Earning and spending per period"
				/>
			</div>
			<div class="gridItem large">
				<CustomLineChart
					type="currencies"
					api_path="/api/v1/reports/balance_over_time_per_currency"
					label_property="symbol"
					title="Balance per Currency over Time"
				/>
			</div>
 			<div class="gridItem large">
				<CustomLineChart
					type="recipients"
					api_path="/api/v1/reports/balance_over_time_per_recipient"
					label_property="name"
					title="Balance per Recipient over Time"
				/>
			</div>
			<div class="gridItem large">
				<CustomLineChart
					type="accounts"
					api_path="/api/v1/reports/balance_over_time_per_account"
					label_property="name"
					title="Balance per Recipient over Time"
				/>
			</div>
		</div>
	</div>
</template>

<script>
export default {
	data: () => ({
		total_per_currency: {},
		show: false
	}),

	async fetch() {
		this.total_per_currency = await this.$axios.$get("/api/v1/reports/total_per_currency");
		this.$nextTick(() => this.show = true); //Important to not error when user without accessToken cookie set visits the index page
	},
}
</script>

<style lang="sass" scoped>

div#grid
	display: grid
	width: 100%
	grid-template-columns: repeat(auto-fit, minmax(10%, 1fr))
	grid-auto-rows: 100px
	align-items: stretch
	justify-items: stretch
	grid-gap: 10px

div.gridItem
	padding: 10px

div.small
	grid-column: span 1
	grid-row: span 1

div.medium
	grid-column: span 2
	grid-row: span 4

div.large
	grid-column: span 4
	grid-row: span 4
	
</style>
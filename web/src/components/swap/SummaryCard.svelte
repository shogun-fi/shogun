<script>
	import { address } from "$stores/wallet";
	import { onMount } from "svelte";

    let orderAmount = "0";
    let cowReceived = "0";
    let sorReceived = "0";
    let savings = "0";
    let receiptDenom = "ATOM";

    async function fetchInfo() {
        const res = await fetch('http://localhost:8000/', {
			method: 'POST',
            body: JSON.stringify({
                "address": $address
            }),
            mode: 'no-cors'
		});

		const json = await res.json()

        orderAmount = json.orderAmount
        cowReceived = json.cowReceived
        sorReceived = json.sorReceived
        savings = json.savings
        receiptDenom = json.denom

    }

    onMount(async () => {
        await fetchInfo()
    })

</script>

<ul class="steps">
    <li class="step step-primary">Matching</li>
    <li class="step step-primary">Routing</li>
    <li class="step step-primary">Summary</li>
</ul>

<div class="divider"></div>

<p class="font-mono text-xs">Order amount: {orderAmount} {receiptDenom}</p>
<p class="font-mono text-xs">Amount received through CoW matching: {cowReceived} {receiptDenom}</p>
<p class="font-mono text-xs">Amount received through smart order routing: {sorReceived} {receiptDenom}</p>
<p class="font-mono text-xs">You saved {savings} {receiptDenom} by using Shogun!</p>
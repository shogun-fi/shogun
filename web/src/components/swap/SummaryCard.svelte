<script>
	import { address } from "$stores/wallet";
	import { onMount } from "svelte";

    async function fetchInfo() {
        const res = await fetch('http://localhost:8000/', {
			method: 'POST',
            body: JSON.stringify({
                "address": $address
            }),
            mode: 'no-cors'
		});

        console.log(await res.text())
		const data = await res.json()

        return data
    }
</script>

<ul class="steps">
    <li class="step step-primary">Matching</li>
    <li class="step step-primary">Routing</li>
    <li class="step step-primary">Summary</li>
</ul>

<div class="divider"></div>

{#await fetchInfo()}
<p class="font-mono text-xs">Loading..</p>
{:then data}
<p class="font-mono text-xs">Order amount: {data.orderAmount} {data.denom}</p>
<p class="font-mono text-xs">Amount received through CoW matching: {data.cowReceived} {data.denom}</p>
<p class="font-mono text-xs">Amount received through smart order routing: {data.sorReceived} {data.denom}</p>
<p class="font-mono text-xs">You saved {data.savings} {data.denom} by using Shogun!</p>
{/await}
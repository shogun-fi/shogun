<script>
	import { address } from "$stores/wallet";
	import { onMount } from "svelte";

    export let progress = 1

    onMount(() => {
        setTimeout(() => {
            progress = 3;
        }, 30000);
    })
</script>

<ul class="steps">
    <li class="step step-primary">Matching</li>
    <li class="step">Routing</li>
    <li class="step">Summary</li>
</ul>

<div class="divider"></div>

<p class="font-mono text-xs">Waiting to be included in batch..</p>
<!-- <p class="font-mono text-xs">Order included in batch, acquiring price fix..</p>
<p class="font-mono text-xs">Batch size: 4 | Price: 4.2</p>
<p class="font-mono text-xs">Amount to be settled through CoW: 234.34 | Residual: 126.53</p> -->
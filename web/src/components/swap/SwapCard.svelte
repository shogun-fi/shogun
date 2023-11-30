<script lang="ts">
	import PairSelector from "./PairSelector.svelte";
    import {v4 as uuidv4 } from "uuid"; 
    import { DirectSecp256k1HdWallet } from "@cosmjs/proto-signing"
	import { address } from "$stores/wallet";

    export let progress = 0

    let amount = "";
    
    let from_denom = "ATOM";
    let to_denom = "stATOM";

    async function submitOrder() {
        const wallet: DirectSecp256k1HdWallet = await DirectSecp256k1HdWallet.generate(24)
        const accounts = await wallet.getAccounts()

        var id = accounts[0].address;
        $address = id;
        const res = await fetch('http://localhost:8000/', {
			method: 'POST',
            body: JSON.stringify({
                'address': id,
                'from_denom': from_denom,
                'to_denom': to_denom,
                'amount': amount
            }),
            mode: 'no-cors'
		})

        progress = 1
    }
</script>

<PairSelector bind:from_token={from_denom} bind:to_token={to_denom}/>
<input type="text" placeholder="0" class="flex-grow input input-lg" bind:value={amount} />
<div class="justify-center card-actions">
    <button class="font-light tracking-wider text-md btn btn-primary btn-block" on:click={submitOrder} >Swap</button> 
</div> 
<!-- <p class="font-mono text-xs text-center text-red-500">• NOT CONNECTED</p> -->
<script>
	import TokenSelector from "./TokenSelector.svelte";
    import ChainSelector from "./TokenSelector.svelte";

    export let from_token = ""
    export let to_token= ""

    function switchPair() {
        let old_from = from_token
        let old_to = to_token
        from_token = old_to
        to_token = old_from
    }
</script>
<div class="flex flex-col w-full lg:flex-row">
    <TokenSelector network="" token={from_token}/>
    <button class="text-xs divider lg:divider-horizontal" on:click={switchPair} >TO</button> 
    <TokenSelector network="" token={to_token}/> 
</div>
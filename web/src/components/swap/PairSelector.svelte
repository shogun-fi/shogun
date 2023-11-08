<script>
	import TokenSelector from "./TokenSelector.svelte";
import ChainSelector from "./TokenSelector.svelte";

</script>
<div class="flex flex-col w-full lg:flex-row">
    <TokenSelector />
    <div class="text-xs divider lg:divider-horizontal">TO</div> 
    <TokenSelector /> 
</div>
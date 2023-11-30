<script>
	import MatchingCard from "$components/swap/MatchingCard.svelte";
	import PairSelector from "$components/swap/PairSelector.svelte";
	import RoutingCard from "$components/swap/RoutingCard.svelte";
	import SummaryCard from "$components/swap/SummaryCard.svelte";
	import SwapCard from "$components/swap/SwapCard.svelte";

    let progress = 0;

</script>

<div class="flex flex-col justify-center h-screen">
    <div class="container mx-auto">
        <div class="flex items-center justify-center">
        <div>
            <div class="shadow-xl bg-base-200 card w-96">
            <div class="card-body">
                <div class="flex flex-col w-full space-y-5">
                    {#if progress > 0 && progress < 3}
                    <div class="flex items-center justify-center">
                        <span class="loading loading-spinner loading-md"></span>
                    </div>
                    {/if}
                    {#if progress == 0}
                    <SwapCard bind:progress={progress} />
                    {/if}
                    {#if progress == 1}
                    <MatchingCard bind:progress={progress} />
                    {/if}
                    {#if progress == 2}
                    <RoutingCard/>
                    {/if}
                    {#if progress == 3}
                    <SummaryCard/>
                    {/if}
                </div>
            </div>
            </div>
        </div>
        </div>
    </div>
</div>
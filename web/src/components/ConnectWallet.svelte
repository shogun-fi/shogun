<script>

    import { createWeb3Modal, defaultWagmiConfig } from '@web3modal/wagmi'

    import { mainnet, arbitrum } from 'viem/chains'

    // 1. Define constants
    const projectId = '238ba446951df90d04677ec0c785a835'

    // 2. Create wagmiConfig
    const metadata = {
    name: 'Shogun',
    description: 'Web3Modal Example',
    url: 'https://shogun.fi',
    icons: ['https://avatars.githubusercontent.com/u/37784886']
    }

    const chains = [mainnet, arbitrum]
    const wagmiConfig = defaultWagmiConfig({ chains, projectId, metadata })

    // 3. Create modal
    const modal = createWeb3Modal({ wagmiConfig, projectId, chains })

    function buttonClicked() {
        modal.open()
    }
</script>
<button class="text-sm font-bold tracking-wider text-accent btn btn-ghost" on:click={buttonClicked}>
    Connect
</button>
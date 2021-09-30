<template>
    <div class="flex flex-row w-full">
        <q-menu class="w-2/12"/>
        <div class="w-10/12">
            <q-topbar />
            <q-main />
        </div>
    </div>
</template>

<script>
import QMenu from './QMenu/index.vue'
import QTopbar from './QTopbar/index.vue'
import QMain from './QMain/index.vue'

export default {
    components: {
        QMenu,
        QTopbar,
        QMain
    }
}
</script>

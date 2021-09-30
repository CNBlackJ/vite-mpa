<template>
    <div class="flex flex-row">
        <img :src="icon" alt="">
        <span>{{ title }}</span>
    </div>
</template>

<script>
export default {
    name: 'MenuItem',
    props: {
        title: String,
        icon: String
    }
}
</script>
<template>
    <div>
        <div v-for="(menuGroup, i) in menus" :key="i">
            <span>{{ menuGroup.title }}</span>
            <menu-item v-for="(menu, m) in menuGroup.children" :key="m" v-bind="menu"></menu-item>
        </div>
    </div>
</template>

<script>
import MenuItem from './MenuItem.vue'

export default {
    components: {
        MenuItem
    },
    data() {
        return {
            menus: [
                {
                    type: 'online',
                    sort: 1,
                    title: '在线音乐',
                    children: [
                        { title: '推荐', icon: '' },
                        { title: '音乐馆', icon: '' },
                        { title: '电台', icon: '' },
                        { title: '视频', icon: '' }
                    ]
                },
                {
                    type: 'my',
                    sort: 2,
                    title: '我的音乐',
                    children: [
                        { title: '我喜欢', icon: '' },
                        { title: '本地歌曲', icon: '' },
                        { title: '下载歌曲', icon: '' },
                        { title: '最近播放', icon: '' }
                    ]
                }
            ]
        }
    }
}
</script>
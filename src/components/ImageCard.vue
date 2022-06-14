<template>
  <div>
    <n-card
      size="small"
      hoverable
    >
      <template #cover>
        <img class="image-card-cover" v-lazy="convertFileSrc(imagesMeta.cover)" @click="browseImages">
      </template>
      <template #header>
        <span class="image-card-title" @click="browseImages">{{ imagesMeta.title }}</span>
      </template>
      <div>
        <n-text type="info" v-if="imagesMeta.author">
          {{ imagesMeta.author }}
        </n-text>
        <div class="image-card-intro" v-if="imagesMeta.intro">
          <n-blockquote align-text>{{ imagesMeta.intro }}</n-blockquote>
        </div>
      </div>
      <template #action>
        <n-space justify="space-around" :size="10">
          <n-button text @click="emit('update', Object.assign({}, imagesMeta))">
            <template #icon>
              <n-icon>
                <Pencil />
              </n-icon>
            </template>
            编辑
          </n-button>
          <n-popconfirm
            @positive-click="emit('remove', imagesMeta.id)"
            positive-text="没错"
            negative-text="算了"
          >
            <template #trigger>
              <n-button text type="error">
                <template #icon>
                  <n-icon>
                    <TrashBin />
                  </n-icon>
                </template>
                移除
              </n-button>
            </template>
            真的要移除吗？
          </n-popconfirm>
        </n-space>
      </template>
    </n-card>
  </div>
</template>

<script>
import { convertFileSrc } from '@tauri-apps/api/tauri'
import { NCard, NBlockquote, NText, NIcon, NButton, NSpace, NPopconfirm } from 'naive-ui'
import { useRouter } from 'vue-router'
import { Pencil, TrashBin } from '@vicons/ionicons5'

export default {
  name: 'ImageCard',
  components: {
    NCard,
    NBlockquote,
    NText,
    NIcon,
    NButton,
    NSpace,
    Pencil,
    TrashBin,
    NPopconfirm
  },
  props: {
    imagesMeta: Object
  },
  emits: ['remove', 'update'],
  setup (props, context) {
    const router = useRouter()

    async function browseImages () {
      router.push({
        path: '/browse',
        query: {
          id: props.imagesMeta.id
        }
      })
    }

    return {
      emit: context.emit,
      convertFileSrc,
      browseImages
    }
  }
}
</script>

<style scoped>
.image-card-intro {
  font-size: 0.9em;
}
.image-card-cover {
  max-width: 100%;
}
.image-card-cover:hover, .image-card-title:hover {
  cursor: pointer;
}
</style>

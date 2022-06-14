<template>
  <div>
    <div style="margin: 20px 0;">
      <n-space justify="center" :size="30">
        <n-input
          round
          placeholder="搜索标题或作者"
          v-model:value="data.search"
          style="width: 400px;"
          @keyup="onSearchInputKeyUp"
          @clear="() => {data.search = ''; getImagesMetaList();}"
          clearable
        >
          <template #suffix>
            <n-icon><Search /></n-icon>
          </template>
        </n-input>
        <n-button strong secondary round type="primary" @click="showAddImagesMeta">
          <template #icon>
            <n-icon>
              <images-outline />
            </n-icon>
          </template>
          导入图片
        </n-button>
      </n-space>
    </div>
    <div v-if="data.metaList.length === 0" style="margin-top: 78px;">
      <n-empty description="现在这里还没有内容">
        <template #extra>
          <n-button strong secondary round type="primary" @click="data.modal.visible = true">
            <template #icon>
              <n-icon>
                <images-outline />
              </n-icon>
            </template>
            导入图片
          </n-button>
        </template>
      </n-empty>
    </div>
    <div v-else style="margin: 12px;">
      <n-grid :x-gap="12" :y-gap="8" cols="1 2xs:1 xs:2 s:4 m:5 l:6 xl:7 2xl:8" responsive="screen">
        <n-grid-item v-for="meta in data.metaList" :key="meta.id">
          <image-card
            :images-meta="meta"
            @remove="removeImagesMeta"
            @update="showUpdateImagesMeta"
          />
        </n-grid-item>
      </n-grid>
      <n-space justify="end" style="margin-top: 20px;">
        <n-pagination
          v-model:page="data.pagination.current"
          :item-count="data.pagination.total"
          v-model:page-size="data.pagination.pageSize"
          :page-sizes="[10, 20, 30, 50]"
          show-size-picker
        />
      </n-space>
    </div>

    <n-modal v-model:show="data.modal.visible">
      <n-card
        style="width: 650px;"
        :title="`${data.modal.form.id ? '更新' : '导入'}图片`"
        :bordered="false"
        size="huge"
        role="dialog"
        aria-modal="true"
      >
        <template #header-extra>
          <n-button strong secondary round type="primary" @click="selectFolder">
            <template #icon>
              <n-icon>
                <folder-open-outline />
              </n-icon>
            </template>
            选择目录
          </n-button>
        </template>
        <n-grid :cols="7" :x-gap="20">
          <n-grid-item span="4">
            <n-form ref="imagesMetaForm" :model="data.modal.form" :rules="data.modal.rules">
              <n-form-item path="title" label="标题">
                <n-input v-model:value="data.modal.form.title" @keydown.enter.prevent />
              </n-form-item>
              <n-form-item path="author" label="作者">
                <n-input v-model:value="data.modal.form.author" @keydown.enter.prevent />
              </n-form-item>
              <n-form-item path="intro" label="简介">
                <n-input v-model:value="data.modal.form.intro" type="textarea" @keydown.enter.prevent />
              </n-form-item>
            </n-form>
          </n-grid-item>
          <n-grid-item span="3">
            <img :src="data.modal.coverAsset" class="modal-cover" @click="changeCover" />
          </n-grid-item>
        </n-grid>
        <template #footer>
          <span style="float: right;">
            <n-button v-if="data.modal.form.id" strong round type="primary" @click="updateImagesMeta">更新</n-button>
            <n-button v-else strong round type="primary" @click="addImagesMeta">添加</n-button>
          </span>
        </template>
      </n-card>
    </n-modal>
  </div>
</template>

<script>
import { reactive, ref, watch } from 'vue'
import { dialog } from '@tauri-apps/api'
import { NButton, NEmpty, NModal, NCard, NForm, NFormItem, NInput, NGrid, NGridItem, NIcon, NSpace, NPagination, useMessage } from 'naive-ui'
import $backend from '../backend'
import { FolderOpenOutline, ImagesOutline, Search } from '@vicons/ionicons5'
import { convertFileSrc } from '@tauri-apps/api/tauri'
import ImageCard from '../components/ImageCard.vue'

export default {
  name: 'MyLibrary',
  components: {
    ImageCard,
    FolderOpenOutline,
    ImagesOutline,
    NButton,
    NEmpty,
    NModal,
    NCard,
    NForm,
    NFormItem,
    NInput,
    NGrid,
    NGridItem,
    NIcon,
    NSpace,
    NPagination,
    Search
  },
  setup () {
    const message = useMessage()

    const data = reactive({
      metaList: [],
      search: '',
      pagination: {
        current: 1,
        pageSize: 50,
        total: 0
      },
      modal: {
        visible: false,
        form: {
          id: null,
          title: '',
          author: '',
          intro: ''
        },
        rules: {
          title: { required: true, trigger: ['focus', 'input', 'blur'], message: '请输入标题' }
        },
        path: '',
        cover: '',
        coverAsset: ''
      }
    })
    const imagesMetaForm = ref(null)

    function initModal () {
      // init
      data.modal = {
        visible: false,
        form: {
          id: null,
          title: '',
          author: '',
          intro: ''
        },
        rules: {
          title: { required: true, trigger: ['focus', 'input', 'blur'], message: '请输入标题' }
        },
        path: '',
        cover: '',
        coverAsset: ''
      }
    }

    async function getImagesMetaList () {
      const page = data.pagination.current
      const pageSize = data.pagination.pageSize
      return $backend.getImagesMetaList(data.search.trim(), page, pageSize).then(res => {
        console.log('getImagesMetaList', res)
        data.metaList = res.list
        data.pagination.current = res.pagination.current
        data.pagination.pageSize = res.pagination.page_size
        data.pagination.total = res.pagination.total
      })
    }

    watch(
      () => [data.pagination.current, data.pagination.pageSize],
      getImagesMetaList
    )

    getImagesMetaList()

    async function selectFolder () {
      dialog.open({
        title: '选择图片目录',
        directory: true
      }).then(path => {
        console.log('选择目录结果', path)
        return $backend.getImagesFolderInfo(path).then(res => {
          console.log('getImagesFolderInfo', path, res)
          function getCover () {
            for (let f of res.files) {
              if (!f.is_dir) return f.path
            }
            for (let f of res.files) {
              if (f.is_dir) {
                for (let ff of f.files) {
                  return ff
                }
              }
            }
          }

          data.modal.cover = getCover()
          data.modal.coverAsset = convertFileSrc(data.modal.cover)
          data.modal.path = path
          data.modal.form.title = res.name
        })
      })
    }

    async function addImagesMeta () {
      return new Promise((resolve, reject) => {
        imagesMetaForm.value?.validate((errors) => {
          if (!errors) {
            const meta = {
              path: data.modal.path,
              title: data.modal.form.title,
              author: data.modal.form.author || '',
              intro: data.modal.form.intro || '',
              cover: data.modal.cover
            }
            $backend.addImagesMeta(meta).then(res => {
              console.log('addImagesMeta', res)
              resolve(res)
            }, err => {
              reject(err)
            })
          }
        })
      }).then(() => {
        initModal()
        return getImagesMetaList()
      })
    }

    async function changeCover () {
      dialog.open({
        title: '选择封面',
        directory: false,
        defaultPath: data.modal.path,
        filters: [{
          name: 'image',
          extensions: ['jpeg', 'jpg', 'webp', 'gif', 'png', 'bmp']
        }]
      }).then(path => {
        data.modal.cover = path
        data.modal.coverAsset = convertFileSrc(data.modal.cover)
      })
    }

    async function removeImagesMeta (metaId) {
      console.log('removeImagesMeta', metaId)
      return $backend.deleteImageMeta(metaId).then(() => {
        return getImagesMetaList()
      }, err => {
        message.error(`移除失败：${err}`)
      })
    }

    function showUpdateImagesMeta (meta) {
      console.log('showUpdateImagesMeta', meta)
      data.modal = {
        visible: true,
        form: {
          id: meta.id,
          title: meta.title,
          author: meta.author,
          intro: meta.intro
        },
        rules: {
          title: { required: true, trigger: ['focus', 'input', 'blur'], message: '请输入标题' }
        },
        path: meta.path,
        cover: meta.cover,
        coverAsset: convertFileSrc(meta.cover)
      }
    }

    function showAddImagesMeta () {
      initModal()
      data.modal.visible = true
    }

    async function updateImagesMeta () {
      return new Promise((resolve, reject) => {
        imagesMetaForm.value?.validate((errors) => {
          if (!errors) {
            const meta = {
              id: data.modal.form.id,
              path: data.modal.path,
              title: data.modal.form.title,
              author: data.modal.form.author || '',
              intro: data.modal.form.intro || '',
              cover: data.modal.cover
            }
            $backend.updateImagesMeta(meta).then(res => {
              console.log('updateImagesMeta', res)
              resolve(res)
            }, err => {
              reject(err)
            })
          }
        })
      }).then(() => {
        initModal()
        return getImagesMetaList()
      })
    }

    function onSearchInputKeyUp (e) {
      console.log('onSearchInputKeyUp', e)
      if (e.keyCode === 13) {
        // enter
        getImagesMetaList()
      }
    }

    return {
      imagesMetaForm,
      data,
      getImagesMetaList,
      selectFolder,
      addImagesMeta,
      changeCover,
      removeImagesMeta,
      showUpdateImagesMeta,
      showAddImagesMeta,
      updateImagesMeta,
      onSearchInputKeyUp
    }
  }
}
</script>

<style scoped>
.modal-cover {
  max-width: 100%;
}
.modal-cover:hover {
  cursor: pointer;
}
</style>

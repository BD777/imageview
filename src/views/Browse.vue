<template>
  <div>
    <div class="top-hover-area">
      <n-button text class="back-button" @click="goBack">
        <template #icon>
          <n-icon size="30"><ArrowBackCircleOutline /></n-icon>
        </template>
      </n-button>
    </div>
    <n-layout has-sider sider-placement="right" class="screen-height">
      <n-layout-content class="screen-height">
        <n-layout has-sider class="screen-height">
          <n-layout-sider
            collapse-mode="transform"
            :collapsed-width="4"
            :width="240"
            :show-collapsed-content="false"
            show-trigger="arrow-circle"
            :collapsed="data.chaptersCollapsed"
            content-style="padding: 0px 24px;"
            :native-scrollbar="false"
            @collapse="data.chaptersCollapsed = true"
            @expand="data.chaptersCollapsed = false"
          >
            <div>
              <n-divider title-placement="left">
                <n-text depth="3">
                  浏览设置
                </n-text>
              </n-divider>
              <div>
                <n-radio-group v-model:value="data.browseSettings.browseType" size="small">
                  <n-radio
                    style="padding: 2px 5px;"
                    v-for="t in data.browseSettings.browseTypes"
                    :key="t.type"
                    :value="t.type"
                    :label="t.name"
                  />
                </n-radio-group>
                <n-switch v-model:value="data.browseSettings.homePage" style="margin-top: 15px;" :disabled="data.browseSettings.browseType in {'single': 1, 'scroll': 1}">
                  <template #checked>
                    有首页
                  </template>
                  <template #unchecked>
                    无首页
                  </template>
                </n-switch>
              </div>
              <n-divider title-placement="left">
                <n-text depth="3">
                  子目录
                </n-text>
              </n-divider>
              <n-list v-if="data.folderInfo.files">
                <n-list-item>
                  <n-text
                    class="chapter-item"
                    :type="`${data.currentPath === '' ? 'info' : 'default'}`"
                    @click="selectChapter('')"
                  >
                    [默认] ({{ data.folderInfo.files.filter(f => !f.is_dir).length }}P)
                  </n-text>
                </n-list-item>
                <n-list-item v-for="f in data.folderInfo.files.filter(f => f.is_dir)" :key="f.path">
                  <n-text
                    class="chapter-item"
                    :type="`${data.currentPath === f.path ? 'info' : 'default'}`"
                    @click="selectChapter(f.path)"
                  >
                    {{ f.name }} ({{ f.files.length }}P)
                  </n-text>
                </n-list-item>
              </n-list>
            </div>
          </n-layout-sider>
          <n-layout-content @click="data.chaptersCollapsed = true" :native-scrollbar="false" ref="contentRef">
            <template v-if="data.browseSettings.browseType === 'single'">
              <n-carousel
                ref="carouselRef"
                effect="fade"
                :slides-per-view="1"
                style="height: 100vh;"
                mousewheel
                direction="vertical"
                :show-dots="false"
                :loop="false"
                @update:current-index="onCurrentIndex"
              >
                <n-carousel-item class="center" v-for="d in currentImages" :key="d.index">
                  <img
                    class="carousel-img"
                    v-lazy="convertFileSrc(d.path)"
                  >
                </n-carousel-item>
              </n-carousel>
            </template>
            <template v-else-if="data.browseSettings.browseType === 'right-left'">
              <n-carousel
                ref="doublePageCarouselRef"
                effect="fade"
                :slides-per-view="1"
                style="height: 100vh;"
                mousewheel
                direction="vertical"
                :show-dots="false"
                :loop="false"
                @update:current-index="onDoublePageCurrentIndex"
              >
                <n-carousel-item class="center" v-for="d in currentImagesDoublePage" :key="d[0].index">
                  <div class="carousel-double-img-container">
                    <img 
                      v-if="d.length > 1"
                      class="carousel-double-img"
                      v-lazy="convertFileSrc(d[1].path)"
                    />
                    <img
                      :class="`${d.length === 1 ? 'carousel-img' : 'carousel-double-img'}`"
                      v-lazy="convertFileSrc(d[0].path)"
                    />
                  </div>
                </n-carousel-item>
              </n-carousel>
            </template>
            <template v-else-if="data.browseSettings.browseType === 'left-right'">
              <n-carousel
                ref="doublePageCarouselRef"
                effect="fade"
                :slides-per-view="1"
                style="height: 100vh;"
                mousewheel
                direction="vertical"
                :show-dots="false"
                :loop="false"
                @update:current-index="onDoublePageCurrentIndex"
              >
                <n-carousel-item class="center" v-for="d in currentImagesDoublePage" :key="d[0].index">
                  <div class="carousel-double-img-container">
                    <img
                      :class="`${d.length === 1 ? 'carousel-img' : 'carousel-double-img'}`"
                      v-lazy="convertFileSrc(d[0].path)"
                    />
                    <img 
                      v-if="d.length > 1"
                      class="carousel-double-img"
                      v-lazy="convertFileSrc(d[1].path)"
                    />
                  </div>
                </n-carousel-item>
              </n-carousel>
            </template>
            <template v-else-if="data.browseSettings.browseType === 'scroll'">
              <div style="margin: 0 auto;">
                <img
                  v-for="d in currentImages"
                  :key="d.index"
                  class="scroll-img"
                  :id="`scroll-img-${d.index}`"
                  v-lazy="convertFileSrc(d.path)"
                />
              </div>
            </template>
          </n-layout-content>
        </n-layout>
      </n-layout-content>
      <n-layout-sider
        ref="siderRef"
        :width="220"
        :native-scrollbar="false"
        bordered
        content-style="padding: 20px;"
      >
        <n-space vertical>
          <div v-for="d in currentImages" :key="d.index" :id="`preview-${d.index}`">
            <img
              :class="`${data.activeImageIndex === d.index ? 'preview-img-active' : 'preview-img'}`"
              v-lazy="convertFileSrc(d.path)"
              @click="onPreviewImageClick(d.index)"
            >
          </div>
        </n-space>
      </n-layout-sider>
    </n-layout>
  </div>
</template>

<script>
import { convertFileSrc } from '@tauri-apps/api/tauri'
import { reactive, computed, ref, watch, nextTick } from 'vue'
import { useRouter } from 'vue-router'
import { NLayout, NLayoutSider, NLayoutContent, NList, NListItem, NText, NCarousel, NCarouselItem, NSpace, NRadioGroup, NRadio, NDivider, NSwitch, NIcon, NButton } from 'naive-ui'
import $backend from '../backend'
import { ArrowBackCircleOutline } from '@vicons/ionicons5'

export default {
  name: 'ImageBrowser',
  components: {
    NLayout,
    NLayoutSider,
    NLayoutContent,
    NList,
    NListItem,
    NText,
    NCarousel,
    NCarouselItem,
    NSpace,
    NRadioGroup,
    NRadio,
    NDivider,
    NSwitch,
    ArrowBackCircleOutline,
    NIcon,
    NButton
  },
  setup () {
    let data = reactive({
      meta: {},
      folderInfo: {},
      currentPath: '',
      chaptersCollapsed: true,
      activeImageIndex: 0,
      browseSettings: {
        browseType: 'single',
        browseTypes: [
          { type: 'single', name: '单页' },
          { type: 'scroll', name: '滚动' },
          { type: 'right-left', name: '右左' },
          { type: 'left-right', name: '左右' }
        ],
        homePage: false
      }
    })
    const currentImages = computed(() => {
      if (!data.folderInfo.files) return []
      if (data.currentPath === '') {
        return data.folderInfo.files.filter(f => !f.is_dir).map((f, index) => {
          return { path: f.path, index: index }
        })
      } else {
        let tmp = data.folderInfo.files.filter(f => f.path === data.currentPath)[0]
        if (!tmp) return []
        return tmp.files.map((path, index) => {
          return { path: path, index: index }
        })
      }
    })
    const currentImagesDoublePage = computed(() => {
      if (currentImages.value.length === 0) return []
      let resp = []
      if (data.browseSettings.homePage) {
        resp.push([currentImages.value[0]])
      }
      for (let i = data.browseSettings.homePage ? 1 : 0; i < currentImages.value.length; i += 2) {
        let tmp = [currentImages.value[i]]
        if (i + 1 < currentImages.value.length) tmp.push(currentImages.value[i+1])
        resp.push(tmp)
      }
      return resp
    })
    const carouselRef = ref(null)
    const doublePageCarouselRef = ref(null)
    const siderRef = ref(null)
    const contentRef = ref(null)

    const router = useRouter()
    const query = router.currentRoute.value.query

    async function getImagesMeta () {
      return $backend.getImagesMeta(parseInt(query.id)).then(res => {
        console.log('getImagesMeta', res)
        data.meta = res
      })
    }

    async function getImagesFolderInfo (path) {
      return $backend.getImagesFolderInfo(path).then(res => {
        console.log('getImagesFolderInfo', res)
        data.folderInfo = res
        if (data.folderInfo.files.filter(f => !f.is_dir).length === 0) {
          if (data.folderInfo.files.length > 0) data.currentPath = data.folderInfo.files[0].path
        }
      })
    }

    async function getBrowseSettings (metaId) {
      return $backend.getBrowseSettings(metaId).then(res => {
        console.log('getBrowseSettings', metaId, res)
        data.browseSettings.browseType = res.browse_type
        data.browseSettings.homePage = res.home_page
        data.currentPath = res.current_path
        // data.activeImageIndex = res.current_index
        nextTick(() => {
          setTimeout(() => {
            updateActiveImageIndex(res.current_index)
          }, 200)
        })
      }, err => {
        console.warn('getBrowseSettings err', metaId, err)
      }).then(() => {
        // watch after update
        watch(
          () => [data.browseSettings.browseType, data.browseSettings.homePage, data.currentPath, data.activeImageIndex], 
          ([browseType, homePage, currentPath, currentIndex]) => {
            // console.log('watch', browseType, homePage, currentPath, currentIndex)
            if (!data.meta.id) return
            $backend.updateBrowseSettings(data.meta.id, browseType, homePage, currentPath, currentIndex).then(() => {
              // console.log('updateBrowseSettings', data.meta.id, browseType, homePage, currentPath, currentIndex, res)
            }, () => {
              // console.error('updateBrowseSettings fail', data.meta.id, browseType, homePage, currentPath, currentIndex, err)
            })
        })
      })
    }

    function selectChapter (path) {
      data.currentPath = path
      if (data.browseSettings.browseType === 'single') {
        carouselRef.value.to(0)
      } else if (data.browseSettings.browseType === 'left-right' || data.browseSettings.browseType === 'right-left') {
        doublePageCarouselRef.value.to(0)
      }
    }

    function onCurrentIndex (currentIndex) {
      // console.log('onCurrentIndex', currentIndex, lastIndex)
      data.activeImageIndex = currentIndex
      const id = `preview-${data.activeImageIndex}`
      const elem = document.getElementById(id)
      siderRef.value.scrollTo({ top: elem.offsetTop - 5, behavior: 'smooth' })
    }

    function onDoublePageCurrentIndex (currentIndex) {
      console.log('onDoublePageCurrentIndex', currentIndex)
      data.activeImageIndex = currentImagesDoublePage.value[currentIndex][0].index
      const id = `preview-${data.activeImageIndex}`
      const elem = document.getElementById(id)
      siderRef.value.scrollTo({ top: elem.offsetTop - 5, behavior: 'smooth' })
    }

    function onPreviewImageClick (imageIndex) {
      data.activeImageIndex = imageIndex
      if (data.browseSettings.browseType === 'single') {
        carouselRef.value.to(imageIndex)
      } else if (data.browseSettings.browseType === 'left-right' || data.browseSettings.browseType === 'right-left') {
        let idx
        if (data.browseSettings.homePage) idx = parseInt((imageIndex + 1) / 2)
        else idx = parseInt(imageIndex / 2)
        doublePageCarouselRef.value.to(idx)
      } else if (data.browseSettings.browseType === 'scroll') {
        const id = `scroll-img-${imageIndex}`
        const elem = document.getElementById(id)
        contentRef.value.scrollTo({ top: elem.offsetTop, behavior: 'smooth' })
      }
    }

    function updateActiveImageIndex (imageIndex) {
      // 更新当前看到的图片下标，并滑动到相应位置
      data.activeImageIndex = imageIndex
      // preview
      const id = `preview-${data.activeImageIndex}`
      const elem = document.getElementById(id)
      siderRef.value.scrollTo({ top: elem.offsetTop - 5, behavior: 'smooth' })
      // main
      onPreviewImageClick(imageIndex)
    }

    function goBack () {
      router.back()
    }

    return {
      data,
      currentImages,
      currentImagesDoublePage,
      convertFileSrc,
      selectChapter,
      onCurrentIndex,
      onDoublePageCurrentIndex,
      carouselRef,
      doublePageCarouselRef,
      siderRef,
      contentRef,
      onPreviewImageClick,
      getImagesMeta,
      getImagesFolderInfo,
      getBrowseSettings,
      goBack
    }
  },
  mounted () {
    // console.log('mounted', this)
    this.getImagesMeta().then(() => {
      this.getImagesFolderInfo(this.data.meta.path)
      this.getBrowseSettings(this.data.meta.id)
    })
  }
}
</script>

<style scoped>
.screen-height {
  height: 100vh;
}
.chapter-item:hover {
  cursor: pointer;
  text-decoration: dotted underline;
}
.carousel-img {
  margin: 0 auto;
  max-width: 100%;
  max-height: 100vh;
  object-fit: contain;
}
.carousel-double-img-container {
  margin: 0 auto;
  max-width: 100%;
  max-height: 100vh;
}
.carousel-double-img {
  object-fit: contain;
  max-width: 50%;
  max-height: 100vh;
}
.scroll-img {
  display: block;
  object-fit: contain;
  max-width: 70%;
  margin: 0 auto;
}
.preview-img {
  margin: 0 auto;
  max-width: 100%;
  max-height: 100%;
  object-fit: contain;
  filter: brightness(60%);
}
.preview-img-active {
  margin: 0 auto;
  max-width: 100%;
  max-height: 100%;
  object-fit: contain;
}
.preview-img:hover {
  cursor: pointer;
  filter: brightness(80%);
}
.center {
  display: flex;
  justify-content: center;
  align-items: center;
}
.top-hover-area {
  height: 60px;
  width: 100%;
  position: fixed;
  top: 0;
  z-index: 100;
  display: flex;
  align-items: center;
}
.top-hover-area:hover > .back-button {
  margin-left: 20px;
  opacity: 1;
}
.back-button {
  margin-left: 20px;
  opacity: 0;
  transition: visibility 0s, opacity 0.1s linear;
  -webkit-transition: visibility 0s, opacity 0.1s linear; /* Safari */
}
</style>

# Changelog

## [0.5.2](https://github.com/xmc-rs/xmc4-hal/compare/v0.5.1...v0.5.2) (2024-04-29)


### Bug Fixes

* **deps:** update rust crate cortex-m-rt to 0.7.4 ([#246](https://github.com/xmc-rs/xmc4-hal/issues/246)) ([3efc4a9](https://github.com/xmc-rs/xmc4-hal/commit/3efc4a9388ea53c485bd1e404f74a098787899f6))

## [0.5.1](https://github.com/xmc-rs/xmc4-hal/compare/v0.5.0...v0.5.1) (2024-02-06)


### Bug Fixes

* **deps:** update rust crate xmc4100 to 0.9.0 ([#224](https://github.com/xmc-rs/xmc4-hal/issues/224)) ([3918993](https://github.com/xmc-rs/xmc4-hal/commit/391899372ea1b4e87aa1f71b5aa45634ca4d2e1c))
* **deps:** update rust crate xmc4200 to 0.9.0 ([#220](https://github.com/xmc-rs/xmc4-hal/issues/220)) ([6cc8da0](https://github.com/xmc-rs/xmc4-hal/commit/6cc8da0f0e77c2b3e4d774f87335178c057968ba))
* **deps:** update rust crate xmc4400 to 0.9.0 ([#225](https://github.com/xmc-rs/xmc4-hal/issues/225)) ([c2b05e0](https://github.com/xmc-rs/xmc4-hal/commit/c2b05e0a2eedcf871d4d58281c7339e4d6b2276a))
* **deps:** update rust crate xmc4800 to 0.9.0 ([#227](https://github.com/xmc-rs/xmc4-hal/issues/227)) ([01c2d2b](https://github.com/xmc-rs/xmc4-hal/commit/01c2d2bd8f3793f06d1e6440b696b79826f7bcec))
* removing unneeded modules ([c0ac0b9](https://github.com/xmc-rs/xmc4-hal/commit/c0ac0b94a82404b3f6babea9c5267718561f3cbc))
* removing unused mods ([843dab6](https://github.com/xmc-rs/xmc4-hal/commit/843dab6f50db0f377a45537dabfb3db7bcfb9ad5))

## [0.5.0](https://github.com/xmc-rs/xmc4-hal/compare/v0.4.0...v0.5.0) (2023-12-30)


### Features

* **flash:** adding apis ([#218](https://github.com/xmc-rs/xmc4-hal/issues/218)) ([3dd196e](https://github.com/xmc-rs/xmc4-hal/commit/3dd196e4f908ac8e2e6fd087f4d86ca93c423c7d))
* **pref:** adidng methods found in flash module ([ebb69c7](https://github.com/xmc-rs/xmc4-hal/commit/ebb69c762b515f25759a35aca548f98c09ec4396))

## [0.4.0](https://github.com/xmc-rs/xmc4-hal/compare/v0.3.4...v0.4.0) (2023-12-27)


### Features

* **scu:** adding gate locked check ([d11f76e](https://github.com/xmc-rs/xmc4-hal/commit/d11f76ec2ed921cafb30bf4aa3b287c4f40578ec))
* **scu:** implementing clock gating ([3cc7cd1](https://github.com/xmc-rs/xmc4-hal/commit/3cc7cd12642e53723c985ad6eba945acb25ba34d))


### Bug Fixes

* **delay:** adding debug derive to struct ([f2fdbe5](https://github.com/xmc-rs/xmc4-hal/commit/f2fdbe5a1307358df51a591b046aca79b0568201))
* inlining function ([bd0191a](https://github.com/xmc-rs/xmc4-hal/commit/bd0191aee73d981f154978ef38fd33c1b4a10c09))
* **rtc:** adding debug derive to struct ([076316c](https://github.com/xmc-rs/xmc4-hal/commit/076316cdc30cb0dd7e229d1df962bb79ed3a46ca))
* **rtc:** cleaning up enums ([3b68c04](https://github.com/xmc-rs/xmc4-hal/commit/3b68c04149c98f46566af9b5ca050117113974b3))
* **scu:** adding debug derive to struct ([157b1a6](https://github.com/xmc-rs/xmc4-hal/commit/157b1a6a5501b9b5d3ea22739dcba505f83660dd))
* **scu:** blocking function from xmc4500 ([225806c](https://github.com/xmc-rs/xmc4-hal/commit/225806cb6a6d4ed9cd17546d1dda0bfc8a735fff))
* **scu:** cleaning from impls ([0808f34](https://github.com/xmc-rs/xmc4-hal/commit/0808f3480ef37bb9a64f06a283eb0651613c612f))
* **time:** replacing into with from ([c02dcae](https://github.com/xmc-rs/xmc4-hal/commit/c02dcaeeeb3a711f7175f4c733587797c87527f2))
* **wdt:** cleaning up watchdog implementation ([df01e4f](https://github.com/xmc-rs/xmc4-hal/commit/df01e4fcb720065587a8330e34b1a458d95a772a))

## [0.3.4](https://github.com/xmc-rs/xmc4-hal/compare/v0.3.3...v0.3.4) (2023-12-26)


### Bug Fixes

* **deps:** update rust crate xmc4100 to 0.8.3 ([#208](https://github.com/xmc-rs/xmc4-hal/issues/208)) ([a65f008](https://github.com/xmc-rs/xmc4-hal/commit/a65f0087a0fea77489a70e6c80a6ae1933e7e44c))
* **deps:** update rust crate xmc4300 to 0.8.0 ([#212](https://github.com/xmc-rs/xmc4-hal/issues/212)) ([a8a9e92](https://github.com/xmc-rs/xmc4-hal/commit/a8a9e924b3f76c7c88cb834f1bcd27dfc61f17ea))
* **deps:** update rust crate xmc4400 to 0.8.3 ([#209](https://github.com/xmc-rs/xmc4-hal/issues/209)) ([769c219](https://github.com/xmc-rs/xmc4-hal/commit/769c21983a9be37fba282c4f11039690e3f1818f))
* **deps:** update rust crate xmc4500 to 0.8.3 ([#210](https://github.com/xmc-rs/xmc4-hal/issues/210)) ([28df9fb](https://github.com/xmc-rs/xmc4-hal/commit/28df9fbdd5090f20ebd766ea2d63b91af618f5cc))
* **deps:** update rust crate xmc4700 to 0.8.3 ([#211](https://github.com/xmc-rs/xmc4-hal/issues/211)) ([7de62ce](https://github.com/xmc-rs/xmc4-hal/commit/7de62ce535201a3a4266da014eee21507903bfb2))
* **deps:** update rust crate xmc4800 to 0.8.0 ([#213](https://github.com/xmc-rs/xmc4-hal/issues/213)) ([4e456a9](https://github.com/xmc-rs/xmc4-hal/commit/4e456a9133fc275c98048934cd5403d788c91fb3))
* **rtc:** adding argument for pac registers ([d689e08](https://github.com/xmc-rs/xmc4-hal/commit/d689e081f8d9d2c6ff31455b1f062cf76339ea2a))

## [0.3.3](https://github.com/xmc-rs/xmc4-hal/compare/v0.3.2...v0.3.3) (2023-12-25)


### Bug Fixes

* **deps:** update rust crate xmc4100 to 0.8.2 ([#201](https://github.com/xmc-rs/xmc4-hal/issues/201)) ([60ea253](https://github.com/xmc-rs/xmc4-hal/commit/60ea2538cdb4c4c75cc2133da1aa986a0228a8f2))
* **deps:** update rust crate xmc4300 to 0.7.2 ([#202](https://github.com/xmc-rs/xmc4-hal/issues/202)) ([c7e031d](https://github.com/xmc-rs/xmc4-hal/commit/c7e031da1ca26faaabb84b11b365a1a3c0e8c7ab))
* **deps:** update rust crate xmc4400 to 0.8.2 ([#203](https://github.com/xmc-rs/xmc4-hal/issues/203)) ([c92678b](https://github.com/xmc-rs/xmc4-hal/commit/c92678bf260576df5dee0f0df1ecf5c923c6fbfa))
* **deps:** update rust crate xmc4500 to 0.8.2 ([#204](https://github.com/xmc-rs/xmc4-hal/issues/204)) ([230b8ec](https://github.com/xmc-rs/xmc4-hal/commit/230b8ecafe6cec1ae0b69394a0d4022f7a555933))
* **deps:** update rust crate xmc4700 to 0.8.2 ([#205](https://github.com/xmc-rs/xmc4-hal/issues/205)) ([a11ac90](https://github.com/xmc-rs/xmc4-hal/commit/a11ac906a85de6990af8692c6b74d283a152a493))
* **deps:** update rust crate xmc4800 to 0.7.2 ([#206](https://github.com/xmc-rs/xmc4-hal/issues/206)) ([4870003](https://github.com/xmc-rs/xmc4-hal/commit/48700038d56b493fd6936e4095677d6104becc36))

## [0.3.2](https://github.com/xmc-rs/xmc4-hal/compare/v0.3.1...v0.3.2) (2023-12-25)


### Bug Fixes

* **deps:** update rust crate xmc4200 to 0.8.2 ([#199](https://github.com/xmc-rs/xmc4-hal/issues/199)) ([82a2325](https://github.com/xmc-rs/xmc4-hal/commit/82a2325d53009fdec76535ce69ca7d75d64b9522))

## [0.3.1](https://github.com/xmc-rs/xmc4-hal/compare/v0.3.0...v0.3.1) (2023-12-23)


### Bug Fixes

* adding rt feature ([#197](https://github.com/xmc-rs/xmc4-hal/issues/197)) ([c0afe22](https://github.com/xmc-rs/xmc4-hal/commit/c0afe22f343690345400ad74c7d1de52c11e0647))

## [0.3.0](https://github.com/xmc-rs/xmc4-hal/compare/v0.2.1...v0.3.0) (2023-12-22)


### Features

* cleaning up feature names ([#195](https://github.com/xmc-rs/xmc4-hal/issues/195)) ([930af83](https://github.com/xmc-rs/xmc4-hal/commit/930af833bab3913295a3f6254ce2a60295da7972))


### Bug Fixes

* **deps:** update rust crate xmc4400 to 0.8.1 ([#192](https://github.com/xmc-rs/xmc4-hal/issues/192)) ([4a560e0](https://github.com/xmc-rs/xmc4-hal/commit/4a560e0191fc6969754836340d673699988bf952))
* **deps:** update rust crate xmc4500 to 0.8.1 ([#193](https://github.com/xmc-rs/xmc4-hal/issues/193)) ([a455b90](https://github.com/xmc-rs/xmc4-hal/commit/a455b902826fe1f34c60b6566b9cdbe749758ba3))
* **deps:** update rust crate xmc4800 to 0.7.1 ([#194](https://github.com/xmc-rs/xmc4-hal/issues/194)) ([2911d20](https://github.com/xmc-rs/xmc4-hal/commit/2911d20b6faef96a458703ea600da08faa55ea1c))

## [0.2.1](https://github.com/xmc-rs/xmc4-hal/compare/v0.2.0...v0.2.1) (2023-12-22)


### Bug Fixes

* **deps:** update rust crate xmc4700 to 0.8.1 ([#190](https://github.com/xmc-rs/xmc4-hal/issues/190)) ([b400793](https://github.com/xmc-rs/xmc4-hal/commit/b400793afaddf6a93e8d38fbe64f7b1320ec1f96))

## [0.2.0](https://github.com/xmc-rs/xmc4-hal/compare/v0.1.1...v0.2.0) (2023-12-22)


### Features

* **scu:** adding more apis ([#184](https://github.com/xmc-rs/xmc4-hal/issues/184)) ([ffac23c](https://github.com/xmc-rs/xmc4-hal/commit/ffac23c02c67bedddcc3490538d8fbf5b756fa2b))


### Bug Fixes

* **deps:** update rust crate xmc4100 to 0.8.1 ([#186](https://github.com/xmc-rs/xmc4-hal/issues/186)) ([f662299](https://github.com/xmc-rs/xmc4-hal/commit/f6622998eb4a29027bcc9ee671e56f2ae80162a1))
* **deps:** update rust crate xmc4200 to 0.8.1 ([#187](https://github.com/xmc-rs/xmc4-hal/issues/187)) ([31b88ed](https://github.com/xmc-rs/xmc4-hal/commit/31b88ed78832de45c0503e5421ac4bc8fbb37a00))
* **deps:** update rust crate xmc4300 to 0.7.1 ([#188](https://github.com/xmc-rs/xmc4-hal/issues/188)) ([a90bd9e](https://github.com/xmc-rs/xmc4-hal/commit/a90bd9e940593f20dec5f4d14b04cd03e9cb5aa7))
* removin unused macro ([9f59b18](https://github.com/xmc-rs/xmc4-hal/commit/9f59b183df6b38c7bf19bc9715f0310f5875c6af))
* **rtc:** removed macro usage ([e53c053](https://github.com/xmc-rs/xmc4-hal/commit/e53c0539a7dc9228c6a19b7e14132d153ca05ffa))
* **scu:** adding implementation to comparator enable/disable ([bf26e45](https://github.com/xmc-rs/xmc4-hal/commit/bf26e451521993181141ceed320f02acb9d0c539))
* **scu:** adding placeholders for temperature functions ([f511e51](https://github.com/xmc-rs/xmc4-hal/commit/f511e51d08a276bdfb6098aa95d7adcd6d3961b0))
* **scu:** improving comparator matches ([6b21885](https://github.com/xmc-rs/xmc4-hal/commit/6b21885013dcc3aa0ac347041306c3144c60dd12))

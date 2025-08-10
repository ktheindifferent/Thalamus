# TODO List

## In Progress Features
- [ ] Capabilities framework for nodes
- [ ] Neural Style Transfer implementation
- [ ] YOLOv3 Darknet Support
- [ ] Move llama to 7B only by default, allow enabling 13B, 30B, 65B via the API
- [ ] YOLOv8 Image Recognition
- [ ] Who.io facial recognition
- [ ] SPREC speech recognition

## High Priority Tasks
- [ ] Complete job queue system implementation
  - [ ] Clear local jobs and inform p2p network to clear them on server boot
  - [ ] Update p2p network with new jobs as they are created and completed
  - [ ] Use job to wrap calculate_stats, nodex, llama, stt, etc.
- [ ] Add encryption support for wav/response
- [ ] Patch Linux to 1.1 version of llama
- [ ] Add support for 13B, 30B, and 65B LLaMA models

## Testing & Quality
- [ ] Create unit tests for core modules
- [ ] Create integration tests for API endpoints
- [ ] Add tests for P2P networking functions
- [ ] Test service discovery mechanisms
- [ ] Validate AI service integrations
- [ ] Performance benchmarking

## Documentation
- [ ] Complete API documentation
- [ ] Add usage examples
- [ ] Document P2P protocol
- [ ] Create deployment guide
- [ ] Add troubleshooting guide

## Infrastructure
- [ ] Implement automatic updates
- [ ] Docker support improvements (brew install --cask docker)
- [ ] Fetch fresh stats and re-average them
- [ ] Check for missing capabilities based on stats

## Future Features
- [ ] Rust-bert support (translation, GPT, summarization)
- [ ] Ability to opt-in to send training data to the Open Sam Foundation
- [ ] Advanced load balancing algorithms
- [ ] Health monitoring and alerting
- [ ] Web UI for node management

## Completed Features ✓
- [x] OpenTTS support
- [x] YOLOv7 integration
- [x] Configurable web pool size, port, etc.
- [x] Basic P2P mesh networking
- [x] mDNS service discovery
- [x] Project structure setup
- [x] Core module architecture

## Recent Accomplishments
- [x] Created project_description.md with current work summary
- [x] Created overview.md with high-level project architecture
- [x] Documented TODO items from codebase
- [x] Identified partially implemented features needing completion
- [x] Fixed dependency issues (wasm-bindgen 0.2.86 -> 0.2.100)
- [x] Fixed time crate issues (0.3.21 -> 0.3.41)
- [x] Added unit test suite with 8 tests for core structures
- [x] Updated project documentation with latest work

## Current Blockers
- [x] ~~PyTorch (libtorch) dependency not installed - blocking build completion~~ (RESOLVED)
  - ✅ Made tch dependency optional with feature flags
  - ✅ Added conditional compilation for NST module
  - ✅ Project now builds without PyTorch

## Next Immediate Actions
1. Run build and check for compilation errors
2. Identify and fix any broken functionality
3. Create basic test suite structure
4. Complete one partially implemented feature
5. Add tests for completed feature
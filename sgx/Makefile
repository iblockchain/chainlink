# Makefile settings -----

ENVIRONMENT ?= release

# SGX SDK settings ------

SGX_SDK ?= /opt/intel/sgxsdk
SGX_MODE ?= HW
SGX_ARCH := x64

# Flags -----------------

SGX_EDGER8R := $(SGX_SDK)/bin/x64/sgx_edger8r
SGX_COMMON_CFLAGS := -m64
RUST_FLAGS :=
ifeq ($(ENVIRONMENT), debug)
	SGX_COMMON_CFLAGS += -O0 -g
else
	SGX_COMMON_CFLAGS += -O2
	RUST_FLAGS := --release
endif

INCLUDES := -I $(SGX_SDK)/include
CC_FLAGS := $(SGX_COMMON_CFLAGS) -fPIC -Wno-attributes $(INCLUDES)

#
# Target aliases --------
#

ENCLAVE := target/$(ENVIRONMENT)/libenclave_u.a
LIBADAPTERS := target/$(ENVIRONMENT)/libadapters.so
UNTRUSTED_EDL_FILES := target/$(ENVIRONMENT)/enclave_u.c target/$(ENVIRONMENT)/enclave_u.h

TRUSTED_EDL_FILES := target/$(ENVIRONMENT)/enclave_t.c target/$(ENVIRONMENT)/enclave_t.h


#
# Targets ---------------
#

.PHONY: all
all: $(LIBADAPTERS)

# Lib Adapters ----------

$(UNTRUSTED_EDL_FILES): $(SGX_EDGER8R) enclave/enclave.edl
	$(SGX_EDGER8R) --untrusted enclave/enclave.edl --search-path $(SGX_SDK)/include --search-path ./edl --untrusted-dir $(@D)

target/$(ENVIRONMENT)/enclave_u.o: $(UNTRUSTED_EDL_FILES)
	$(CC) $(CC_FLAGS) -c $< -o $@

$(ENCLAVE): target/$(ENVIRONMENT)/enclave_u.o
	$(AR) rcsD $@ $^

$(LIBADAPTERS): libadapters/Cargo.toml libadapters/src/*.rs $(ENCLAVE)
	ENVIRONMENT=$(ENVIRONMENT) SGX_SDK=$(SGX_SDK) cargo build $(RUST_FLAGS) --manifest-path $<
	cp libadapters/$(LIBADAPTERS) $@

# Enclave ---------------

clean:
	cd libadapters && cargo clean
	rm -rf target

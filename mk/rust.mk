RUSTC						=	rustc
RUSTDOC						=	rustdoc

BUILDDIR					:=	.build
LIBDIR						:=	lib

RUSTCFLAGS					:=	-L $(LIBDIR)
RUSTDOCFLAGS				:=

## UTILS
# Recursive wildcard function
# http://blog.jgc.org/2011/07/gnu-make-recursive-wildcard-function.html
rwildcard=$(foreach d,$(wildcard $1*),$(call rwildcard,$d/,$2) \
  $(filter $(subst *,%,$2),$d))

map = $(foreach a,$(2),$(call $(1),$(a)))

######################################################################
define MODULE_RULES

$(1)_PATH					:=	src/$(1)
$(1)_SOURCES				:=	$$(call rwildcard,$$($(1)_PATH),*.rs)
$(1)_DEPS_NAMES				:=	$$(foreach dep,$$($(1)_DEPS),$(BUILDDIR)/.build_$$(dep))

ifneq ($$(wildcard $$($(1)_PATH)/main.rs),)
$(1)_TYPE					:=	bin
$(1)_MAIN_SOURCE			:=	$$($(1)_PATH)/main.rs
else ifneq ($$(wildcard $$($(1)_PATH)/lib.rs),)
$(1)_TYPE					:=	lib
$(1)_MAIN_SOURCE			:=	$$($(1)_PATH)/lib.rs
$(1)_LIB					:=	$$(wildcard $(LIBDIR)/lib$(1)-*.so)
else
$$(error Unkown module type: $(1))
endif

ifneq ($$(wildcard $$($(1)_PATH)/test.rs),)
$(1)_TESTNAME				:=	$(BUILDDIR)/test_$(1)
else
$(1)_TESTNAME				:=
endif

$(1):						$(BUILDDIR)/.build_$(1)
.PHONY:						$(1)

$(BUILDDIR)/.build_$(1):	$$($(1)_DEPS_NAMES) $$($(1)_SOURCES)
ifeq ($$($(1)_TYPE),bin)
	$$(RUSTC) $$(RUSTCFLAGS) -o $(1) $$($(1)_MAIN_SOURCE)
else ifeq ($$($(1)_TYPE),lib)
	@mkdir -p $(LIBDIR)
	$$(RUSTC) $$(RUSTCFLAGS) --lib --out-dir $(LIBDIR) $$($(1)_MAIN_SOURCE)
endif
	@touch $(BUILDDIR)/.build_$(1)

clean_$(1):
ifeq ($$($(1)_TYPE),bin)
	@rm -f $(1)
else ifeq ($$($(1)_TYPE),lib)
	@rm -f $$($(1)_LIB)
endif
	@rm -f $(BUILDDIR)/.build_$(1)
.PHONY:						clean_$(1)

test_$(1):					$$($(1)_TESTNAME)
ifneq ($$(wildcard $$($(1)_PATH)/test.rs),)
	@$$($(1)_TESTNAME)
endif

bench_$(1):					$$($(1)_TESTNAME)
ifneq ($$(wildcard $$($(1)_PATH)/test.rs),)
	@$$($(1)_TESTNAME) --bench
endif

ifneq ($$(wildcard $$($(1)_PATH)/test.rs),)
$$($(1)_TESTNAME):			$$($(1)_SOURCES)
	$$(RUSTC) $$(RUSTCFLAGS) --test -o $$($(1)_TESTNAME) $$($(1)_PATH)/test.rs
endif

endef
######################################################################

all:						$(BUILDDIR) $(MODULES)

clean:						$(addprefix clean_,$(MODULES))
	@rm -rf $(BUILDDIR) $(LIBDIR)

test:						$(addprefix test_,$(MODULES))

bench:						$(addprefix bench_,$(MODULES))

$(foreach mod,$(MODULES),$(eval $(call MODULE_RULES,$(mod))))

$(BUILDDIR):
	@mkdir -p $(BUILDDIR)

.PHONY:						all clean test bench

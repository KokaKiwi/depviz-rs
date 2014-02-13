RUSTC				=	rustc

DEBUG				=	0

RUSTCFLAGS			=
ifeq ($(DEBUG),1)
RUSTCFLAGS			+=	-Z debug-info -Z extra-debug-info
else
RUSTCFLAGS			+=	--opt-level=3
endif

# Rules
all:				_librustdeps _depviz

clean:

## UTILS
# Recursive wildcard function
# http://blog.jgc.org/2011/07/gnu-make-recursive-wildcard-function.html
rwildcard=$(foreach d,$(wildcard $1*),$(call rwildcard,$d/,$2) \
  $(filter $(subst *,%,$2),$d))

# librustdeps
RUSTDEPS_SRCDIR		=	src/rustdeps
RUSTDEPS_ROOT		=	$(RUSTDEPS_SRCDIR)/lib.rs
RUSTDEPS_SRCS		=	$(call rwildcard,$(RUSTDEPS_SRCDIR),*.rs)

RUSTDEPS_FLAGS		=	--crate-type=dylib
RUSTDEPS_NAMES		=	$(shell $(RUSTC) --crate-file-name $(RUSTDEPS_FLAGS) $(RUSTDEPS_ROOT))
RUSTDEPS_TARGET		=	$(firstword $(RUSTDEPS_NAMES))

_librustdeps:		$(RUSTDEPS_TARGET)

$(RUSTDEPS_TARGET):	$(RUSTDEPS_SRCS)
	$(RUSTC) $(RUSTCFLAGS) $(RUSTDEPS_FLAGS) $(RUSTDEPS_ROOT)

clean_librustdeps:
	rm -f $(RUSTDEPS_NAMES)
clean:				clean_librustdeps

# depviz
DEPVIZ_SRCDIR		=	src/depviz
DEPVIZ_ROOT			=	$(DEPVIZ_SRCDIR)/main.rs
DEPVIZ_SRCS			=	$(call rwildcard,$(DEPVIZ_SRCDIR),*.rs)

DEPVIZ_FLAGS		=	-L .
DEPVIZ_NAMES		=	$(shell $(RUSTC) --crate-file-name $(DEPVIZ_FLAGS) $(DEPVIZ_ROOT))
DEPVIZ_TARGET		=	$(firstword $(DEPVIZ_NAMES))

_depviz:			$(DEPVIZ_TARGET)

$(DEPVIZ_TARGET):	$(RUSTDEPS_NAMES) $(DEPVIZ_SRCS)
	$(RUSTC) $(RUSTCFLAGS) $(DEPVIZ_FLAGS) $(DEPVIZ_ROOT)

clean_depviz:
	rm -f $(DEPVIZ_NAMES)
clean:				clean_depviz

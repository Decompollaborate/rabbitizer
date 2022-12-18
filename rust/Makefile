RS_TABLE_GEN    := ../tools/rs_table_gen.sh

IINC            := -I ../include

TABLE_DIRS      := $(shell find src -type d)
TABLE_TEMPLATES := $(foreach dir,$(TABLE_DIRS),$(wildcard $(dir)/*.tablers.template))
TABLE_GENERATED := $(TABLE_TEMPLATES:%.tablers.template=%.rs)

TABLE_DEP_FILES := $(TABLE_GENERATED:%.rs=%.tablers.d)


all: $(TABLE_GENERATED)

distclean:
	$(RM) -rf $(TABLE_GENERATED)


.PHONY: all distclean
.DEFAULT_GOAL := all
.SECONDARY:


%.rs: %.tablers.template
	cpp -P $(IINC) -M -MM -MMD -MP -MF $(@:.rs=.tablers.d) $<
	$(RS_TABLE_GEN) $< $@

-include $(TABLE_DEP_FILES)

# Print target for debugging
print-% : ; $(info $* is a $(flavor $*) variable set to [$($*)]) @true

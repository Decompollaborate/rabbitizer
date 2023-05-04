.section .text

.type count_leading_one,@function
.globl count_leading_one
count_leading_one:
    clo         $v0, $a0 #
    jr          $ra
.size count_leading_one, . - count_leading_one


.type count_leading_zero,@function
.globl count_leading_zero
count_leading_zero:
    clz         $v0, $a0 #
    jr          $ra
.size count_leading_zero, . - count_leading_zero


.type multiply_add,@function
.globl multiply_add
multiply_add:
	mult        $a0, $a1
    madd        $a2, $a3 #
    mflo        $v0
    jr          $ra
.size multiply_add, . - multiply_add


.type multiply_add_unsigned,@function
.globl multiply_add_unsigned
multiply_add_unsigned:
	mult        $a0, $a1
    maddu       $a2, $a3 #
    mflo        $v0
    jr          $ra
.size multiply_add_unsigned, . - multiply_add_unsigned


.type multiply_subtract,@function
.globl multiply_subtract
multiply_subtract:
	mult        $a0, $a1
    msub        $a2, $a3 #
    mflo        $v0
    jr          $ra
.size multiply_subtract, . - multiply_subtract


.type multiply_subtract_unsigned,@function
.globl multiply_subtract_unsigned
multiply_subtract_unsigned:
	mult        $a0, $a1
    msubu       $a2, $a3 #
    mflo        $v0
    jr          $ra
.size multiply_subtract_unsigned, . - multiply_subtract_unsigned




.type select_max,@function
.globl select_max
select_max:
    max         $v0, $a0, $a1 #
    jr          $ra
.size select_max, . - select_max


.type select_min,@function
.globl select_min
select_min:
    min         $v0, $a0, $a1 #
    jr          $ra
.size select_min, . - select_min


.type move_conditional_on_not_zero,@function
.globl move_conditional_on_not_zero
move_conditional_on_not_zero:
    li          $t0, -1
    move        $v0, $t0
    movn        $v0, $a0, $a1 #
    jr          $ra
.size move_conditional_on_not_zero, . - move_conditional_on_not_zero


.type move_conditional_on_zero,@function
.globl move_conditional_on_zero
move_conditional_on_zero:
    li          $t0, -1
    move        $v0, $t0
    movz        $v0, $a0, $a1 #
    jr          $ra
.size move_conditional_on_zero, . - move_conditional_on_zero




.type extract_bit_field_2_5,@function
.globl extract_bit_field_2_5
extract_bit_field_2_5:
    ext         $v0, $a0, 2, 5 #
    jr          $ra
.size extract_bit_field_2_5, . - extract_bit_field_2_5


.type extract_bit_field_7_5,@function
.globl extract_bit_field_7_5
extract_bit_field_7_5:
    ext         $v0, $a0, 7, 5 #
    jr          $ra
.size extract_bit_field_7_5, . - extract_bit_field_7_5



.type insert_bit_field_2_5,@function
.globl insert_bit_field_2_5
insert_bit_field_2_5:
    ins         $v0, $a0, 2, 5 #
    jr          $ra
.size insert_bit_field_2_5, . - insert_bit_field_2_5


.type insert_bit_field_7_5,@function
.globl insert_bit_field_7_5
insert_bit_field_7_5:
    ins         $v0, $a0, 7, 5 #
    jr          $ra
.size insert_bit_field_7_5, . - insert_bit_field_7_5


.type sign_extend_byte,@function
.globl sign_extend_byte
sign_extend_byte:
    sll         $a0, $a0, 24
    sra         $a0, $a0, 24
    seb         $v0, $a0 #
    jr          $ra
.size sign_extend_byte, . - sign_extend_byte


.type sign_extend_halfword,@function
.globl sign_extend_halfword
sign_extend_halfword:
    sll         $a0, $a0, 16
    sra         $a0, $a0, 16
    seh         $v0, $a0 #
    jr          $ra
.size sign_extend_halfword, . - sign_extend_halfword



.type bit_reverse,@function
.globl bit_reverse
bit_reverse:
    bitrev      $v0, $a0 #
    jr          $ra
.size bit_reverse, . - bit_reverse



.type rotate_word_right_6,@function
.globl rotate_word_right_6
rotate_word_right_6:
    rotr        $v0, $a0, 6 #
    jr          $ra
.size rotate_word_right_6, . - rotate_word_right_6


.type rotate_word_right_19,@function
.globl rotate_word_right_19
rotate_word_right_19:
    rotr        $v0, $a0, 19 #
    jr          $ra
.size rotate_word_right_19, . - rotate_word_right_19



.type rotate_word_right_variable,@function
.globl rotate_word_right_variable
rotate_word_right_variable:
    rotrv       $v0, $a0, $a1 #
    jr          $ra
.size rotate_word_right_variable, . - rotate_word_right_variable




.type word_swap_bytes_within_halfword,@function
.globl word_swap_bytes_within_halfword
word_swap_bytes_within_halfword:
    wsbh        $v0, $a0 #
    jr          $ra
.size word_swap_bytes_within_halfword, . - word_swap_bytes_within_halfword


.type word_swap_bytes_within_word,@function
.globl word_swap_bytes_within_word
word_swap_bytes_within_word:
    wsbw        $v0, $a0 #
    jr          $ra
.size word_swap_bytes_within_word, . - word_swap_bytes_within_word





.type cache_index_invalidate,@function
.globl cache_index_invalidate
cache_index_invalidate:
    cache       0x04, 0x0($a0) #
    jr          $ra
.size cache_index_invalidate, . - cache_index_invalidate

.type cache_index_unlock,@function
.globl cache_index_unlock
cache_index_unlock:
    cache       0x06, 0x0($a0) #
    jr          $ra
.size cache_index_unlock, . - cache_index_unlock

.type cache_hit_invalidate,@function
.globl cache_hit_invalidate
cache_hit_invalidate:
    cache       0x08, 0x0($a0) #
    jr          $ra
.size cache_hit_invalidate, . - cache_hit_invalidate

.type cache_fill,@function
.globl cache_fill
cache_fill:
    cache       0x0A, 0x0($a0) #
    jr          $ra
.size cache_fill, . - cache_fill

.type cache_fill_with_lock,@function
.globl cache_fill_with_lock
cache_fill_with_lock:
    cache       0x0B, 0x0($a0) #
    jr          $ra
.size cache_fill_with_lock, . - cache_fill_with_lock

.type cache_index_writeback_invalidate,@function
.globl cache_index_writeback_invalidate
cache_index_writeback_invalidate:
    cache       0x14, 0x0($a0) #
    jr          $ra
.size cache_index_writeback_invalidate, . - cache_index_writeback_invalidate

.type cache_index_unlock_D,@function
.globl cache_index_unlock_D
cache_index_unlock_D:
    cache       0x16, 0x0($a0) #
    jr          $ra
.size cache_index_unlock_D, . - cache_index_unlock_D

.type cache_create_dirty_exclusive,@function
.globl cache_create_dirty_exclusive
cache_create_dirty_exclusive:
    cache       0x18, 0x0($a0) #
    jr          $ra
.size cache_create_dirty_exclusive, . - cache_create_dirty_exclusive

.type cache_hit_invalidate_D,@function
.globl cache_hit_invalidate_D
cache_hit_invalidate_D:
    cache       0x19, 0x0($a0) #
    jr          $ra
.size cache_hit_invalidate_D, . - cache_hit_invalidate_D

.type cache_hit_writeback,@function
.globl cache_hit_writeback
cache_hit_writeback:
    cache       0x1A, 0x0($a0) #
    jr          $ra
.size cache_hit_writeback, . - cache_hit_writeback

.type cache_hit_writeback_invalidate,@function
.globl cache_hit_writeback_invalidate
cache_hit_writeback_invalidate:
    cache       0x1B, 0x0($a0) #
    jr          $ra
.size cache_hit_writeback_invalidate, . - cache_hit_writeback_invalidate

.type cache_create_dirty_exclsuive_with_lock,@function
.globl cache_create_dirty_exclsuive_with_lock
cache_create_dirty_exclsuive_with_lock:
    cache       0x1C, 0x0($a0) #
    jr          $ra
.size cache_create_dirty_exclsuive_with_lock, . - cache_create_dirty_exclsuive_with_lock

.type cache_fill_D,@function
.globl cache_fill_D
cache_fill_D:
    cache       0x1E, 0x0($a0) #
    jr          $ra
.size cache_fill_D, . - cache_fill_D

.type cache_fill_with_lock_D,@function
.globl cache_fill_with_lock_D
cache_fill_with_lock_D:
    cache       0x1F, 0x0($a0) #
    jr          $ra
.size cache_fill_with_lock_D, . - cache_fill_with_lock_D




.type synchronize_shared_memory,@function
.globl synchronize_shared_memory
synchronize_shared_memory:
    sync #
    jr          $ra
.size synchronize_shared_memory, . - synchronize_shared_memory



# sets LLbit to 1
.type load_linked,@function
.globl load_linked
load_linked:
    ll          $v0, 0x0($a0) #
    jr          $ra
.size load_linked, . - load_linked




.type store_conditional,@function
.globl store_conditional
store_conditional:
    sc          $a1, 0x0($a0) #
    jr          $ra
.size store_conditional, . - store_conditional






.type all_caches,@function
.globl all_caches
all_caches:
    cache       0x00, 0x0($a0) #
    cache       0x01, 0x0($a0) #
    cache       0x02, 0x0($a0) #
    cache       0x03, 0x0($a0) #
    cache       0x04, 0x0($a0) #
    cache       0x05, 0x0($a0) #
    cache       0x06, 0x0($a0) #
    cache       0x07, 0x0($a0) #
    cache       0x08, 0x0($a0) #
    cache       0x09, 0x0($a0) #
    cache       0x0A, 0x0($a0) #
    cache       0x0B, 0x0($a0) #
    cache       0x0C, 0x0($a0) #
    cache       0x0D, 0x0($a0) #
    cache       0x0E, 0x0($a0) #
    cache       0x0F, 0x0($a0) #
    cache       0x10, 0x0($a0) #
    cache       0x11, 0x0($a0) #
    cache       0x12, 0x0($a0) #
    cache       0x13, 0x0($a0) #
    cache       0x14, 0x0($a0) #
    cache       0x15, 0x0($a0) #
    cache       0x16, 0x0($a0) #
    cache       0x17, 0x0($a0) #
    cache       0x18, 0x0($a0) #
    cache       0x19, 0x0($a0) #
    cache       0x1A, 0x0($a0) #
    cache       0x1B, 0x0($a0) #
    cache       0x1C, 0x0($a0) #
    cache       0x1D, 0x0($a0) #
    cache       0x1E, 0x0($a0) #
    cache       0x1F, 0x0($a0) #
    jr          $ra
.size all_caches, . - all_caches



.type halt,@function
.globl halt
halt:
    sleep       #
    jr          $ra
.size halt, . - halt

.type get_interrupt_state,@function
.globl get_interrupt_state
get_interrupt_state:
    mfie        $v0
    jr          $ra
.size get_interrupt_state, . - get_interrupt_state

.type disable_interrupts,@function
.globl disable_interrupts
disable_interrupts:
    mfie        $zero
    jr          $ra
.size disable_interrupts, . - disable_interrupts

.type set_interrupt_state,@function
.globl set_interrupt_state
set_interrupt_state:
    mtie        $a0
    jr          $ra
.size set_interrupt_state, . - set_interrupt_state


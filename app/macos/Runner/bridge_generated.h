#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
typedef struct _Dart_Handle* Dart_Handle;

typedef struct DartCObject DartCObject;

typedef int64_t DartPort;

typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);

typedef struct DartCObject *WireSyncReturn;

typedef struct wire_RwLockData {
  const void *ptr;
} wire_RwLockData;

typedef struct wire_uint_8_list {
  uint8_t *ptr;
  int32_t len;
} wire_uint_8_list;

typedef struct wire_StringList {
  struct wire_uint_8_list **ptr;
  int32_t len;
} wire_StringList;

typedef struct wire_RwLockState {
  const void *ptr;
} wire_RwLockState;

void store_dart_post_cobject(DartPostCObjectFnType ptr);

Dart_Handle get_dart_object(uintptr_t ptr);

void drop_dart_object(uintptr_t ptr);

uintptr_t new_dart_opaque(Dart_Handle handle);

intptr_t init_frb_dart_api_dl(void *obj);

WireSyncReturn wire_get_dynamic(void);

void wire_get_cached(int64_t port_);

WireSyncReturn wire_get_state(struct wire_RwLockData data,
                              struct wire_StringList *ctx,
                              struct wire_uint_8_list *lang);

void wire_move_to_state(int64_t port_,
                        struct wire_RwLockState state,
                        struct wire_uint_8_list *text,
                        struct wire_RwLockData data);

void wire_back(int64_t port_, struct wire_RwLockState state);

void wire_home(int64_t port_, struct wire_RwLockState state);

WireSyncReturn wire_depth(struct wire_RwLockState state);

WireSyncReturn wire_is_empty(struct wire_RwLockState state);

WireSyncReturn wire_context(struct wire_RwLockState state);

WireSyncReturn wire_get_link(struct wire_RwLockState state);

WireSyncReturn wire_get_message(struct wire_RwLockState state);

WireSyncReturn wire_get_button_texts(struct wire_RwLockState state);

struct wire_RwLockData new_RwLockData(void);

struct wire_RwLockState new_RwLockState(void);

struct wire_StringList *new_StringList_0(int32_t len);

struct wire_uint_8_list *new_uint_8_list_0(int32_t len);

void drop_opaque_RwLockData(const void *ptr);

const void *share_opaque_RwLockData(const void *ptr);

void drop_opaque_RwLockState(const void *ptr);

const void *share_opaque_RwLockState(const void *ptr);

void free_WireSyncReturn(WireSyncReturn ptr);

static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) wire_get_dynamic);
    dummy_var ^= ((int64_t) (void*) wire_get_cached);
    dummy_var ^= ((int64_t) (void*) wire_get_state);
    dummy_var ^= ((int64_t) (void*) wire_move_to_state);
    dummy_var ^= ((int64_t) (void*) wire_back);
    dummy_var ^= ((int64_t) (void*) wire_home);
    dummy_var ^= ((int64_t) (void*) wire_depth);
    dummy_var ^= ((int64_t) (void*) wire_is_empty);
    dummy_var ^= ((int64_t) (void*) wire_context);
    dummy_var ^= ((int64_t) (void*) wire_get_link);
    dummy_var ^= ((int64_t) (void*) wire_get_message);
    dummy_var ^= ((int64_t) (void*) wire_get_button_texts);
    dummy_var ^= ((int64_t) (void*) new_RwLockData);
    dummy_var ^= ((int64_t) (void*) new_RwLockState);
    dummy_var ^= ((int64_t) (void*) new_StringList_0);
    dummy_var ^= ((int64_t) (void*) new_uint_8_list_0);
    dummy_var ^= ((int64_t) (void*) drop_opaque_RwLockData);
    dummy_var ^= ((int64_t) (void*) share_opaque_RwLockData);
    dummy_var ^= ((int64_t) (void*) drop_opaque_RwLockState);
    dummy_var ^= ((int64_t) (void*) share_opaque_RwLockState);
    dummy_var ^= ((int64_t) (void*) free_WireSyncReturn);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    return dummy_var;
}
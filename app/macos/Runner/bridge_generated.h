#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
typedef struct _Dart_Handle* Dart_Handle;

typedef struct DartCObject DartCObject;

typedef int64_t DartPort;

typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);

typedef struct DartCObject *WireSyncReturn;

typedef struct wire_MultilangFs {
  const void *ptr;
} wire_MultilangFs;

typedef struct wire_RwLockFaContext {
  const void *ptr;
} wire_RwLockFaContext;

typedef struct wire_uint_8_list {
  uint8_t *ptr;
  int32_t len;
} wire_uint_8_list;

void store_dart_post_cobject(DartPostCObjectFnType ptr);

Dart_Handle get_dart_object(uintptr_t ptr);

void drop_dart_object(uintptr_t ptr);

uintptr_t new_dart_opaque(Dart_Handle handle);

intptr_t init_frb_dart_api_dl(void *obj);

void wire_get_data(int64_t port_);

WireSyncReturn wire_get_context(void);

void wire_get_fs(int64_t port_, struct wire_MultilangFs mlfs, struct wire_RwLockFaContext ctx);

void wire_transition(int64_t port_, struct wire_RwLockFaContext ctx, struct wire_uint_8_list *text);

void wire_back(int64_t port_, struct wire_RwLockFaContext ctx);

void wire_home(int64_t port_, struct wire_RwLockFaContext ctx);

struct wire_MultilangFs new_MultilangFs(void);

struct wire_RwLockFaContext new_RwLockFaContext(void);

struct wire_uint_8_list *new_uint_8_list_0(int32_t len);

void drop_opaque_MultilangFs(const void *ptr);

const void *share_opaque_MultilangFs(const void *ptr);

void drop_opaque_RwLockFaContext(const void *ptr);

const void *share_opaque_RwLockFaContext(const void *ptr);

void free_WireSyncReturn(WireSyncReturn ptr);

static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) wire_get_data);
    dummy_var ^= ((int64_t) (void*) wire_get_context);
    dummy_var ^= ((int64_t) (void*) wire_get_fs);
    dummy_var ^= ((int64_t) (void*) wire_transition);
    dummy_var ^= ((int64_t) (void*) wire_back);
    dummy_var ^= ((int64_t) (void*) wire_home);
    dummy_var ^= ((int64_t) (void*) new_MultilangFs);
    dummy_var ^= ((int64_t) (void*) new_RwLockFaContext);
    dummy_var ^= ((int64_t) (void*) new_uint_8_list_0);
    dummy_var ^= ((int64_t) (void*) drop_opaque_MultilangFs);
    dummy_var ^= ((int64_t) (void*) share_opaque_MultilangFs);
    dummy_var ^= ((int64_t) (void*) drop_opaque_RwLockFaContext);
    dummy_var ^= ((int64_t) (void*) share_opaque_RwLockFaContext);
    dummy_var ^= ((int64_t) (void*) free_WireSyncReturn);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    return dummy_var;
}

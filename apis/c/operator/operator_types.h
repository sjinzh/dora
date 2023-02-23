/*! \file */
/*******************************************
 *                                         *
 *  File auto-generated by `::safer_ffi`.  *
 *                                         *
 *  Do not manually edit this file.        *
 *                                         *
 *******************************************/

#ifndef __RUST_DORA_OPERATOR_API_C__
#define __RUST_DORA_OPERATOR_API_C__
#ifdef __cplusplus
extern "C" {
#endif


#include <stddef.h>
#include <stdint.h>

/** \brief
 *  Same as [`Vec<T>`][`rust::Vec`], but with guaranteed `#[repr(C)]` layout
 */
typedef struct Vec_uint8 {
    /** <No documentation available> */
    uint8_t * ptr;

    /** <No documentation available> */
    size_t len;

    /** <No documentation available> */
    size_t cap;
} Vec_uint8_t;

/** <No documentation available> */
typedef struct DoraResult {
    /** <No documentation available> */
    Vec_uint8_t error;
} DoraResult_t;

/** <No documentation available> */
typedef struct DoraDropOperator {
    /** <No documentation available> */
    DoraResult_t (*drop_operator)(void *);
} DoraDropOperator_t;

/** <No documentation available> */
typedef struct DoraInitResult {
    /** <No documentation available> */
    DoraResult_t result;

    /** <No documentation available> */
    void * operator_context;
} DoraInitResult_t;

/** <No documentation available> */
typedef struct DoraInitOperator {
    /** <No documentation available> */
    DoraInitResult_t (*init_operator)(void);
} DoraInitOperator_t;

/** <No documentation available> */
/** \remark Has the same ABI as `uint8_t` **/
#ifdef DOXYGEN
typedef
#endif
enum DoraStatus {
    /** <No documentation available> */
    DORA_STATUS_CONTINUE = 0,
    /** <No documentation available> */
    DORA_STATUS_STOP = 1,
    /** <No documentation available> */
    DORA_STATUS_STOP_ALL = 2,
}
#ifndef DOXYGEN
; typedef uint8_t
#endif
DoraStatus_t;

/** <No documentation available> */
typedef struct OnEventResult {
    /** <No documentation available> */
    DoraResult_t result;

    /** <No documentation available> */
    DoraStatus_t status;
} OnEventResult_t;

/** <No documentation available> */
typedef struct Metadata {
    /** <No documentation available> */
    Vec_uint8_t open_telemetry_context;
} Metadata_t;

/** <No documentation available> */
typedef struct Input {
    /** <No documentation available> */
    Vec_uint8_t id;

    /** <No documentation available> */
    Vec_uint8_t data;

    /** <No documentation available> */
    Metadata_t metadata;
} Input_t;


#include <stdbool.h>

/** <No documentation available> */
typedef struct FfiEvent {
    /** <No documentation available> */
    Input_t * input;

    /** <No documentation available> */
    bool stop;
} FfiEvent_t;

/** <No documentation available> */
typedef struct Output {
    /** <No documentation available> */
    Vec_uint8_t id;

    /** <No documentation available> */
    Vec_uint8_t data;

    /** <No documentation available> */
    Metadata_t metadata;
} Output_t;

/** \brief
 *  `Arc<dyn Send + Sync + Fn(A1) -> Ret>`
 */
typedef struct ArcDynFn1_DoraResult_Output {
    /** <No documentation available> */
    void * env_ptr;

    /** <No documentation available> */
    DoraResult_t (*call)(void *, Output_t);

    /** <No documentation available> */
    void (*release)(void *);

    /** <No documentation available> */
    void (*retain)(void *);
} ArcDynFn1_DoraResult_Output_t;

/** <No documentation available> */
typedef struct SendOutput {
    /** <No documentation available> */
    ArcDynFn1_DoraResult_Output_t send_output;
} SendOutput_t;

/** <No documentation available> */
typedef struct DoraOnEvent {
    /** <No documentation available> */
    OnEventResult_t (*on_event)(FfiEvent_t const *, SendOutput_t const *, void *);
} DoraOnEvent_t;


#ifdef __cplusplus
} /* extern \"C\" */
#endif

#endif /* __RUST_DORA_OPERATOR_API_C__ */

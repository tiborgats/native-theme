//
//  Copyright (c) 2014-2015 Apple Inc. All rights reserved.
//

#import <XCUIAutomation/XCUIAutomationDefines.h>
#import <XCUIAutomation/XCUIElement.h>

#if TARGET_OS_OSX
#import <AppKit/AppKit.h>
#endif

#if TARGET_OS_IPHONE
#import <UIKit/UIKit.h>
#endif

NS_ASSUME_NONNULL_BEGIN

#if !TARGET_OS_TV

@class XCUIElement;

/*! A coordinate represents a location on screen, relative to some element. Coordinates are dynamic, just like the elements to which they refer, and may compute different screen locations at different times, or be invalid if the referenced element does not exist. */
XCUI_SWIFT_MAIN_ACTOR
@interface XCUICoordinate : NSObject <NSCopying>

+ (instancetype)new XCUI_UNAVAILABLE("Coordinates are never instantiated directly. Instead, they are created by elements or by other coordinates.");
- (instancetype)init XCUI_UNAVAILABLE("Coordinates are never instantiated directly. Instead, they are created by elements or by other coordinates.");

/*! The element that the coordinate is based on, either directly or via the coordinate from which it was derived. */
@property (readonly) XCUIElement *referencedElement;

/*! The dynamically computed value of the coordinate's location on screen. Note that this value is dependent on the current frame of the referenced element; if the element's frame changes, so will the value returned by this property. If the referenced element does exist when this is called, it will fail the test; check the referenced element's exists property if the element may not be present. */
@property (readonly) CGPoint screenPoint;

/*! Creates a new coordinate with an absolute offset in points from the original coordinate. */
- (XCUICoordinate *)coordinateWithOffset:(CGVector)offsetVector;

@end

@interface XCUICoordinate (XCUICoordinateTouchEvents)

- (void)tap;
- (void)doubleTap;
- (void)pressForDuration:(NSTimeInterval)duration;
- (void)pressForDuration:(NSTimeInterval)duration thenDragToCoordinate:(XCUICoordinate *)otherCoordinate;
- (void)pressForDuration:(NSTimeInterval)duration thenDragToCoordinate:(XCUICoordinate *)otherCoordinate withVelocity:(XCUIGestureVelocity)velocity thenHoldForDuration:(NSTimeInterval)holdDuration;

/*!
 * Sends a swipe-up gesture.
 */
- (void)swipeUp API_AVAILABLE(macCatalyst(13.0)) API_UNAVAILABLE(ios, tvos, watchos);

/*!
 * Sends a swipe-down gesture.
 */
- (void)swipeDown API_AVAILABLE(macCatalyst(13.0)) API_UNAVAILABLE(ios, tvos, watchos);

/*!
 * Sends a swipe-left gesture.
 */
- (void)swipeLeft API_AVAILABLE(macCatalyst(13.0)) API_UNAVAILABLE(ios, tvos, watchos);

/*!
 * Sends a swipe-right gesture.
 */
- (void)swipeRight API_AVAILABLE(macCatalyst(13.0)) API_UNAVAILABLE(ios, tvos, watchos);

/*!
 * Sends a swipe in the specified direction with a specified velocity.
 * @param velocity
 * The velocity pixels per second at which to perform the swipe at.
*/
- (void)swipeUpWithVelocity:(XCUIGestureVelocity)velocity API_AVAILABLE(macCatalyst(13.0)) API_UNAVAILABLE(ios, tvos, watchos);
- (void)swipeDownWithVelocity:(XCUIGestureVelocity)velocity API_AVAILABLE(macCatalyst(13.0)) API_UNAVAILABLE(ios, tvos, watchos);
- (void)swipeLeftWithVelocity:(XCUIGestureVelocity)velocity API_AVAILABLE(macCatalyst(13.0)) API_UNAVAILABLE(ios, tvos, watchos);
- (void)swipeRightWithVelocity:(XCUIGestureVelocity)velocity API_AVAILABLE(macCatalyst(13.0)) API_UNAVAILABLE(ios, tvos, watchos);

@end

#if TARGET_OS_OSX || TARGET_OS_MACCATALYST || TARGET_OS_IOS

XCUI_SWIFT_MAIN_ACTOR
@interface XCUICoordinate (XCUICoordinateMouseEvents)

- (void)hover API_AVAILABLE(ios(15.0), macCatalyst(13.0));
- (void)click API_AVAILABLE(ios(15.0), macCatalyst(13.0));
- (void)doubleClick API_AVAILABLE(ios(15.0), macCatalyst(13.0));
- (void)rightClick API_AVAILABLE(ios(15.0), macCatalyst(13.0));
- (void)clickForDuration:(NSTimeInterval)duration thenDragToCoordinate:(XCUICoordinate *)otherCoordinate API_AVAILABLE(ios(15.0), macCatalyst(13.0));
- (void)clickForDuration:(NSTimeInterval)duration thenDragToCoordinate:(XCUICoordinate *)otherCoordinate withVelocity:(XCUIGestureVelocity)velocity thenHoldForDuration:(NSTimeInterval)holdDuration API_AVAILABLE(ios(15.0), macCatalyst(13.0));
- (void)scrollByDeltaX:(CGFloat)deltaX deltaY:(CGFloat)deltaY API_AVAILABLE(ios(15.0), macCatalyst(13.0));

@end

#endif // TARGET_OS_OSX || TARGET_OS_MACCATALYST || TARGET_OS_IOS

#endif

NS_ASSUME_NONNULL_END

//! `SeaORM` Entity, @generated by sea-orm-codegen 0.1.0

use protobuf_parse::Parser;
pub fn parse_proto_file(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
	let parser = Parser::new().pure();
	parser.inputs(
		"./proto/address.proto",
		"./proto/analytics_config.proto",
		"./proto/batch_job.proto",
		"./proto/cart.proto",
		"./proto/cart_discounts.proto",
		"./proto/cart_gift_cards.proto",
		"./proto/claim_image.proto",
		"./proto/claim_item.proto",
		"./proto/claim_item_tags.proto",
		"./proto/claim_order.proto",
		"./proto/claim_tag.proto",
		"./proto/country.proto",
		"./proto/currency.proto",
		"./proto/custom_shipping_option.proto",
		"./proto/customer.proto",
		"./proto/customer_group.proto",
		"./proto/customer_group_customers.proto",
		"./proto/discount.proto",
		"./proto/discount_condition.proto",
		"./proto/discount_condition_customer_group.proto",
		"./proto/discount_condition_product.proto",
		"./proto/discount_condition_product_collection.proto",
		"./proto/discount_condition_product_tag.proto",
		"./proto/discount_condition_product_type.proto",
		"./proto/discount_regions.proto",
		"./proto/discount_rule.proto",
		"./proto/discount_rule_products.proto",
		"./proto/draft_order.proto",
		"./proto/fulfillment.proto",
		"./proto/fulfillment_item.proto",
		"./proto/fulfillment_provider.proto",
		"./proto/gift_card.proto",
		"./proto/gift_card_transaction.proto",
		"./proto/idempotency_key.proto",
		"./proto/image.proto",
		"./proto/invite.proto",
		"./proto/line_item.proto",
		"./proto/line_item_adjustment.proto",
		"./proto/line_item_tax_line.proto",
		"./proto/money_amount.proto",
		"./proto/note.proto",
		"./proto/notification.proto",
		"./proto/notification_provider.proto",
		"./proto/oauth.proto",
		"./proto/onboarding_state.proto",
		"./proto/order.proto",
		"./proto/order_discounts.proto",
		"./proto/order_edit.proto",
		"./proto/order_gift_cards.proto",
		"./proto/order_item_change.proto",
		"./proto/payment.proto",
		"./proto/payment_collection.proto",
		"./proto/payment_collection_payments.proto",
		"./proto/payment_collection_sessions.proto",
		"./proto/payment_provider.proto",
		"./proto/payment_session.proto",
		"./proto/price_list.proto",
		"./proto/price_list_customer_groups.proto",
		"./proto/product.proto",
		"./proto/product_category.proto",
		"./proto/product_category_product.proto",
		"./proto/product_collection.proto",
		"./proto/product_images.proto",
		"./proto/product_option.proto",
		"./proto/product_option_value.proto",
		"./proto/product_sales_channel.proto",
		"./proto/product_shipping_profile.proto",
		"./proto/product_tag.proto",
		"./proto/product_tags.proto",
		"./proto/product_tax_rate.proto",
		"./proto/product_type.proto",
		"./proto/product_type_tax_rate.proto",
		"./proto/product_variant.proto",
		"./proto/product_variant_inventory_item.proto",
		"./proto/product_variant_money_amount.proto",
		"./proto/publishable_api_key.proto",
		"./proto/publishable_api_key_sales_channel.proto",
		"./proto/refund.proto",
		"./proto/region.proto",
		"./proto/region_fulfillment_providers.proto",
		"./proto/region_payment_providers.proto",
		"./proto/return.proto",
		"./proto/return_item.proto",
		"./proto/return_reason.proto",
		"./proto/sales_channel.proto",
		"./proto/sales_channel_location.proto",
		"./proto/shipping_method.proto",
		"./proto/shipping_method_tax_line.proto",
		"./proto/shipping_option.proto",
		"./proto/shipping_option_requirement.proto",
		"./proto/shipping_profile.proto",
		"./proto/shipping_tax_rate.proto",
		"./proto/staged_job.proto",
		"./proto/store.proto",
		"./proto/store_currencies.proto",
		"./proto/swap.proto",
		"./proto/tax_provider.proto",
		"./proto/tax_rate.proto",
		"./proto/tracking_link.proto",
		"./proto/user.proto",
	);
	parser.include(".");
	Ok(())
}
